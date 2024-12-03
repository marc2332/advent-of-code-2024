const input = await Deno.readTextFile("./input", "UTF-8");

let mul = false;
let a = "";
let comma = false;
let b = "";

let total = 0;

const defaultValues = () => {
  mul = null;
  a = "";
  comma = false;
  b = "";
}

for (const ch of input.trim().replaceAll("mul(", "M").split("")) {
  const c = ch.trim();
  switch (c) {
    case 'M':
      if (!mul && a == "" && !comma && b == "") {
        mul = true;
      } else {
        defaultValues()
      }
      break;
    case ')':
      if (mul && a != "" && comma && b != "") {
        total += Number(a) * Number(b)
      }
      defaultValues()
      break;
    case ',':
      if (mul && a != "" && !comma) {
        comma = true;
      } else {
        defaultValues()
      }
      break;
    default:
      if (!Number.isNaN(Number(c)) && mul && !comma) {
        a = `${a}${c}`;
      } else if (!Number.isNaN(Number(c)) && mul && a != null && comma) {
        b = `${b}${c}`;
      } else {
        defaultValues()
      }
  }
}

console.log(total);