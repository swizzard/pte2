import { renderFile } from "@jenny/mustache";
import { parse } from "@std/toml";

async function main() {
  const exercisesString = await Deno.readTextFile("exercises.toml");
  const exercises = parse(exercisesString);
  const rendered = await renderFile("index.template.html", exercises as object);
  await Deno.writeTextFile("public/index.html", rendered);
}

main();
