import init, { json_data } from "../wasm2/pkg/wasm2.js";

init().then(() => {
  console.log("WASM2:");
  try {
    console.log(json_data([[{ filtertype: "Append", type: "Standard" }]]));
  } catch (e) {
    console.log(e);
  }
});
