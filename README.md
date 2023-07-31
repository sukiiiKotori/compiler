- 开启优化

  在`./src/main.rs`内的``SETTINGS`结构体处，将`optimise`改为`true`。

- 生成llvm IR

  ```bash
  cargo run test.c -llvm test.ll
  ```

- 生成RiscV ASM

  ```bash
  cargo run test.c -S test.s
  ```

- 使用vscode调试

  - 更改`.vscode/launch.json`文件，将`args`改为要编译的`*.c`文件。


  ```json
  {
      "version": "0.2.0",
      "configurations": [
          {
              "name": "rust", // 配置名称，将会在调试配置下拉列表中显示
              "type": "cppvsdbg", // 调试器类型：Windows表示器使用cppvsdbg；GDB和LLDB使用cppdbg。该值自动生成
              "request": "launch", // 调试方式
              "program": "${workspaceRoot}/target/debug/compiler.exe", // 要调试的程序（完整路径，支持相对路径）
              "args": ["${workspaceRoot}/test.c", "-S", "out.s"], // 传递给上面程序的参数，没有参数留空即可
              "stopAtEntry": false, // 是否停在程序入口点（即停在main函数开始）（目前为不停下）
              "cwd": "${workspaceRoot}", // 调试程序时的工作目录
              "preLaunchTask": "build", //预先执行task.json
              //"MIMode": "lldb" //MAC下的debug程序
          }
      ]
  }
  ```

  - 按`f5`启动调试

- 使用`compiler2023大赛官方测例进行功能测试`

  - 目前只支持`macos`/`linux`（或者你的`windows`装了`bash`）

  ```shell
  cd tests
  ./test.sh
  ```

  