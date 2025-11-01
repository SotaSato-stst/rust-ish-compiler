TODO: やることリスト
* 全体のアーキテクチャ決め
* とりあえず簡単なコードのコンパイルをできるようにする
* 機能の肉付け

* 全体のアーキテクチャ
ソースコード -> AST -> THIR -> MIR -> バックエンド(LLVM-IR)
* AST: ソースコードをそのままASTにする
* THIR: type inferenceを行い、完全に肩がついた形
* MIR: データフローグラフ

コンポーネント
* lexer
* parser: 再帰下降パーサー
* AST -> HIR: desugaring
* HIR -> THIR: type inference, type check
* THIR -> MIR:
* MIR -> LLVM IR

* 決めること
* eBNFでASTの形を決める

ASTの実装はできた。
やりたいのは、
* ASTからのtype inference
* desugaring、macro expansion
* control flow graphの解析
* コード生成
* レジスタ割り当て
* 各種最適化

とりあえず、asmファイルの生成ができたが、linkできない状態になっているので、直すところから始める
serializeは、各structにやらしたいところではある

次は、関数呼び出しと、数値演算をやるようにしたい
先にSSA対応した方が、やりやすい可能性はある(後でSSA対応をして恩恵を感じるのも良さそう)
parseからやる
次はcode generation

intrisicなfunctionの生成をやる