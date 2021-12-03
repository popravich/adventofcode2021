import * as wasm from "wasm-adventofcode";


function day1(input, label1, label2) {
  const result = timeit(() => wasm.get_answer(input));
  label1.value = result.result1;
  label2.value = result.result2;
}

function day2(input, label1, label2) {
  const answer = timeit(() => wasm.day2_task(input));
  label1.value = answer.result1;
  label2.value = answer.result2;
}

function installFormHandler(day, fn) {
  const hndl = function(ev) {
    ev.preventDefault();
    const input = ev.target.elements[`day${day}-input`].value.trim();
    const label1 = ev.target.elements[`day${day}-answer1`];
    const label2 = ev.target.elements[`day${day}-answer2`];
    fn(input, label1, label2);
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

function timeit(callback) {
  const t0 = performance.now();
  try {
    return callback();
  } finally {
    const t1 = performance.now();
    console.log(`Operation took ${t1 - t0} milliseconds.`);
  }
}

installFormHandler(1, day1);
installFormHandler(2, day2);
