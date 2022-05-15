import React, { useEffect } from "react" ;
import init, { grayscale } from "picture-wasm" ;

import logo from "./logo.svg";
import "./App.css";

function App() {
  useEffect(() => {
    // wasm初始化，在调用`picture-wasm`包方法时
    // 必须先保证已经进行过初始化，否则会报错
    // 如果存在多个wasm包，则必须对每一个wasm包进行初始化
    init();
  }, []);

  const fileImport = (e: any) => {
    const selectedFile = e.target.files[0];
    //获取读取我文件的File对象
    // var selectedFile = document.getElementById( files ).files[0];
    var reader = new FileReader(); //这是核心,读取操作就是由它完成.
    reader.readAsArrayBuffer(selectedFile); //读取文件的内容,也可以读取文件的URL
    reader.onload = (res: any) => {
      var uint8Array = new Uint8Array(res.target.result as ArrayBuffer);
      grayscale(uint8Array);
    };
  };

  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>Hello WebAssembly!</p>
        <p>Vite + Rust + React</p>
        <input type="file" id="files" onChange={fileImport} />
      </header>
    </div>
  );
}

export default App;