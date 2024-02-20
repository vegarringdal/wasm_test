import init, { json_data } from "../wasm1/pkg/wasm1.js";

init().then(() => {
  console.log("WASM1:");
  try {
    console.log(json_data([[{ filtertype: "Append", type: "Standard" }]]));
  } catch (e) {
    console.log(e);
  }
});
