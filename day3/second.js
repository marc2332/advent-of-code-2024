const input = await Deno.readTextFile("./input", "UTF-8");

let DO = true;
let MUL = false;

let a = "";
let comma = false;
let b = "";

let total = 0;

const defaultValues = () => {
  MUL = null;
  a = "";
  comma = false;
  b = "";
}

for (const c of input.trim().replaceAll("don't", "N").replaceAll("do", "Y").replaceAll("mul(", "M").split("")) {
  switch (c) {
    case 'Y':
      if (!MUL && a == "" && !comma && b == "" && !DO) {
        DO = true;
      } else {
        defaultValues()
      }
      break;
    case 'N':
      if (!MUL && a == "" && !comma && b == "" && DO) {
        DO = false;
      } else {
        defaultValues()
      }
      break;
    case 'M':
      if (!MUL && a == "" && !comma && b == "" && DO) {
        MUL = true;
      } else {
        defaultValues()
      }
      break;
    case ')':
      if (MUL && a != "" && comma && b != "") {
        total += Number(a) * Number(b)
      }
      defaultValues()
      break;
    case ',':
      if (MUL && a != "" && !comma) {
        comma = true;
      } else {
        defaultValues()
      }
      break;
    default:
      if (!Number.isNaN(Number(c)) && MUL && !comma) {
        a = `${a}${c}`;
      } else if (!Number.isNaN(Number(c)) && MUL && a != "" && comma) {
        b = `${b}${c}`;
      } else {
        defaultValues()
      }
  }
}

console.log(total);