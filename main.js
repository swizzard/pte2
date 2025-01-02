const fs = require("node:fs/promises");
const Mustache = require("mustache");
const { parse } = require("toml");

async function main() {
  const exercisesString = await fs.readFile("exercises.toml", "utf-8");
  const exercises = parse(exercisesString);
  const tpl = await fs.readFile("index.template.html", "utf-8");
  const rendered = Mustache.render(tpl, exercises);
  await fs.writeFile("public/index.html", rendered);
}

main();
