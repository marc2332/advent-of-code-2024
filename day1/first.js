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
leftCol.forEach((left, i) => {
    const right = rightCol[i];
    let diff = 0 ;
    if(left > right){
        diff = left - right;
    } else {
        diff = right - left;
    }
    total += diff;
})

console.log(total);