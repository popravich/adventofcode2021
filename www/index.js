import * as wasm from "wasm-adventofcode";


function installFormHandler(day, fn) {
  const hndl = function(ev) {
    ev.preventDefault();
    const input = ev.target.elements[`day${day}-input`].value.trim();
    const label1 = ev.target.elements[`day${day}-answer1`];
    const label2 = ev.target.elements[`day${day}-answer2`];
    const { result1, result2 } = timeit(() => fn(input), `day${day}`);
    label1.value = result1;
    label2.value = result2;
    return false
  };
  const form = document.getElementById(`day${day}-form`);
  const file = document.getElementById(`day${day}-file`);
  const read = function(ev) {
    const files = ev.target.files;
    if(files.length > 0) {
      const reader = new FileReader();
      reader.onload = function(e) {
        form.elements[`day${day}-input`].value = e.target.result;
      }
      reader.readAsText(files[0]);
    }
  };
  form.addEventListener('submit', hndl);
  file.addEventListener('change', read);
}

function timeit(callback, label) {
  console.time(label);
  try {
    return callback();
  } finally {
    console.timeEnd(label);
  }
}


for(let i = 1; i <= document.forms.length; i++){
  installFormHandler(i, wasm[`day${i}_task`]);
}
