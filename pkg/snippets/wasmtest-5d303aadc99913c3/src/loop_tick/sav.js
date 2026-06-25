export function set_val(str, val){
  console.log(val);
  localStorage.setItem(str, val);
}

export function get_val(str){
  if (!localStorage.getItem(str)) {
    localStorage.setItem(str, "{}");
    return "{}";
  } else {
    console.log(localStorage.getItem(str));
    return localStorage.getItem(str);
  }
}