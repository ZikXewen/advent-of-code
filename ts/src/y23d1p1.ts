const file = await Bun.file("input.txt").text();

const answer = file
  .split("\n")
  .map((line) => {
    const digits = line.match(/\d/g) ?? "0";
    return parseInt((digits.at(0) ?? "0") + (digits.at(-1) ?? "0"));
  })
  .reduce((a, b) => a + b, 0);

console.log(answer);
