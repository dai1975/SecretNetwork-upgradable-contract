import { compile, compileFromFile } from 'json-schema-to-typescript';
import * as process from 'process';
import * as path from 'path';
import * as fs from 'fs';

const arg0 = path.basename(process.argv[1]);
const input = process.argv[2];
if (input == null) {
  console.log(`${arg0} <json schema file>`);
  process.exit(0);
}

const f = async (input) => {
  const out = path.join(path.dirname(input), path.basename(input, '.json') + '.d.ts')
  await compileFromFile(input)
    .then(ts => fs.writeFileSync(out, ts))
}

f(input)
