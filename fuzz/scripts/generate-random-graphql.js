import { mkdir, readFile, rm, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { buildSchema, print } from 'graphql';
import { generateRandomMutation, generateRandomQuery } from 'ibm-graphql-query-generator';
import { logUpdateStderr } from 'log-update';

const FUZZ_DIR = path.dirname(import.meta.dirname);
const TEST_DATA_DIR = path.join(FUZZ_DIR, '..', 'packages', 'graphql-minify', 'test_data');
const OUT_DIR = path.join(TEST_DATA_DIR, 'random');
const LIMIT = 500;

await rm(OUT_DIR, { recursive: true, force: true });
await mkdir(OUT_DIR);

const schema = buildSchema(
	await readFile(path.join(TEST_DATA_DIR, 'valid', 'github_schema.graphql'), {
		encoding: 'utf8',
	}),
);

/** @type {import('ibm-graphql-query-generator').Configuration} */
const config = {
	breadthProbability: 0.5,
	depthProbability: 0.5,
	maxDepth: 10,
	providePlaceholders: true,
	argumentsToConsider: ['first'],
	considerUnions: true,
};

/** @type {Set<string>} */
const created = new Set();
let lastReportedProgress = 0,
	skippedDuplicates = 0,
	failures = 0;

printProgress();
const progressInterval = setInterval(() => printProgress(), 250);

while (created.size < LIMIT) {
	const type = created.size % 2 === 0 ? 'query' : 'mutation';
	/** @type {[query: string, seed: number] | undefined} */
	let data;

	try {
		switch (type) {
			case 'query': {
				const { queryDocument, seed } = generateRandomQuery(schema, config);
				data = [print(queryDocument), seed];
				break;
			}

			case 'mutation': {
				const { mutationDocument, seed } = generateRandomMutation(schema, config);
				data = [print(mutationDocument), seed];
				break;
			}
		}
	} catch {}

	if (!data) {
		failures += 1;
		continue;
	}

	const [query, seed] = data;

	if (created.has(query)) {
		skippedDuplicates += 1;
		continue;
	}

	created.add(query);

	const filename = `${type}_${formatSeed(seed)}.graphql`;

	try {
		await writeFile(path.join(OUT_DIR, filename), query, { encoding: 'utf8' });
	} catch (err) {
		console.error(`failed to write ${filename}:`, err);
	}
}

clearInterval(progressInterval);
printProgress();
logUpdateStderr.done();

/**
 * @param {number} seed
 * @returns {string}
 */
function formatSeed(seed) {
	return seed.toPrecision(18).padEnd(23, '0').replace('.', '');
}

function printProgress() {
	const progress = created.size;

	if (progress <= lastReportedProgress) {
		return;
	}

	lastReportedProgress = progress;
	logUpdateStderr(
		`generated: ${progress}/${LIMIT}, skipped duplicates: ${skippedDuplicates}, failures: ${failures}`,
	);
}
