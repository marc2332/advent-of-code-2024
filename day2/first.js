const input = await Deno.readTextFile("./input", "UTF-8");

const results = input.trim().split("\n")
  .map((level, i) => {
    const numbers = level.split(" ").map(Number);
    let prev = null;
    let direction = null;

    for(const num of numbers){
      if(!direction && prev){
        if(num > prev){
          direction = "increasing"
        } else {
          direction = "decreasing"
        }
      }

      const diff = num - prev;

      if(prev && direction){
        if(direction == "decreasing"){
          if(diff > -1 || diff < -3) {
            return "unsafe"
          } 
        } else {
          if(diff < 1 || diff > 3 ) {
            return "unsafe"
          } 
        }
      }

      prev = num;
    }

    return "safe"
});


console.log(results.filter(r => r == "safe").length);