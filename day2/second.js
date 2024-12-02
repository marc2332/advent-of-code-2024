const input = await Deno.readTextFile("./input", "UTF-8");

function checkArray(numbers) {
    const numbersInc = numbers.toSorted((a, b) => a - b);
    const numbersDecr = numbersInc.toReversed();
    const inc = numbers.toString() === numbersInc.toString();
    const decr = numbers.toString() === numbersDecr.toString();

    if (!inc && !decr) {
        return false
    }

    for (let i = 0; i < numbers.length; i++) {
        const num = numbers[i];
        const next = numbers[i + 1]

        const diff = Math.abs(next - num);

        if (diff < 1 || diff > 3) {
            return false;
        }
    }
    
    return true
}

const results = input.trim().split("\n")
    .map((level) => {
        const numbers = level.split(" ").map(Number);
        if(checkArray(numbers)){
            return true;
        }
        for (let n = 0; n < numbers.length; n++) {
            if (checkArray(numbers.filter((_, b) => b != n))) {
                return true
            }
        }
});

console.log(results.filter(Boolean).length);
