const input = await Deno.readTextFile("./input", "UTF-8");

let m = false;
let u = false;
let l = false;
let p0 = false;
let a = null;
let comma = false;
let b = null;

let total = 0;

const defaultValues = () => {
  m = false;
  u = false;
  l = false;
  p0 = false;
  a = null;
  comma = false;
  b = null;
}

for(const ch of input.trim().split("")) {
  const c = ch.trim();
  switch(c){
    case 'm':
      if(!m && !u && !l && !p0 && a == null && !comma && b == null) {
        m = true;
      } else {
        defaultValues()
      }
      break;
    case 'u':
      if(m && !u){
        u = true;
      } else {
        defaultValues()
      }
      break;
    case 'l':
      if(m && u && !l){
        l = true;
      } else {
        defaultValues()
      }
      break;
    case '(':
      if(m && u && l && !p0){
        p0 = true;
      } else {
        defaultValues()
      }
      break;
    case ')':
      if(m && u && l && p0 && a != null && comma && b != null) {
        total += Number(a) * Number(b)
        if(a == "906") {
          console.log("!!!")
        }
        console.log("found",a, b, Number(a) * Number(b))
      }
      defaultValues()
      break;
    case ',':
      if(m && u && l && p0 && a != null && !comma){
        comma = true;
      } else {
        defaultValues()
      }
      break;
    default:
      if(!Number.isNaN(Number(c)) && m && u && l && p0 && !comma) {
        if(a == null){
          a = c;
        } else {
          a = `${a}${c}`;
        }
      } else if(!Number.isNaN(Number(c)) && m && u && l && p0 && a != null && comma) {
        if(b == null){
          b = c;
        } else {
          b = `${b}${c}`;
        }
      } else {
        defaultValues()
      }
      
  }
}


console.log(total);