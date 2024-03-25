// Given an ESM module which returns a config and a language, output them as JSON.

const file = Deno.args[0];
console.info("File: ", file);
const module = await import(`./${file}`);
console.log("Imported module. Result:", module);
const conf = module.conf;
const language = module.language;

console.time("Output");
console.time("Output: Config");
Deno.mkdir(`${file}_out`, { recursive: true });
const p = Deno.writeTextFile(
	`${file}_out/conf.json`,
	JSON.stringify(conf)
).then(() => {
	console.timeEnd("Output: Config");
});
console.time("Output: Language");
const p2 = Deno.writeTextFile(
	`${file}_out/lang.json`,
	JSON.stringify(language)
).then(() => {
	console.timeEnd("Output: Language");
});
Promise.all([p, p2]).then(() => {
	console.timeEnd("Output");
});
