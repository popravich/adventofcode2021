import * as wasm from "wasm-adventofcode";


function getAnswer(inputId, output1, output2) {
  const input = document.getElementById(inputId).value;
  const result = wasm.get_answer(input.trim());
  document.getElementById(output1).value = result.result1;
  document.getElementById(output2).value = result.result2;
}


document.getElementById("btn-1").addEventListener("click", () => getAnswer("input-1", "output-1", "output-2"));
