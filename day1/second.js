const input = await Deno.readTextFile("./input", "UTF-8");

const leftCol = []
const rightCol = []
input.trim().split("\n")
  .forEach((row) => {
    const [left, right] = row.split("   ");
    leftCol.push(Number(left))
    rightCol.push(Number(right))
  });
leftCol.sort();
rightCol.sort();

let total = 0;
rightCol.forEach((right) => {
    let n = 0;
    for(const left of leftCol){
        if(left === right) {
            n += 1;
        }
    }
    total += right * n;
})

console.log(total);