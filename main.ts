// Generate GraphViz files

import { readLines } from "https://deno.land/std@0.148.0/io/mod.ts";

const fileReader = await Deno.open("./test.txt");

const lines = [];

for await (const line of readLines(fileReader)) {
  lines.push(line);
}

const gv = `\
digraph {
  node [shape=circle];
${lines.map((line) => "  " + line[0] + " -> " + line[1]).join("\n")}
}
`;

await Deno.writeTextFile("dot.gv", gv);
