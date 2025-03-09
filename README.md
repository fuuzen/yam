# yam

yam 是 **Y**et **A**nother **M**usic 的缩写，是一个编写乐曲的解释型语言，本项目所构建的程序即为其解释器。

本项目作为一个本科阶段编译原理实验课程作业，专注于简单的单文件编译器构造的前端、中端部分。

## EBNF 语法设计

定义在 `EBNF.xbnf` 中，VSCode 可以使用插件获得语法高亮。

## 后缀名约定

yam 语言的源文件后缀约定为 `.yam`，目前解释出来直接生成 `.mid` SMF 文件。