# Roket API

Rocket フレームワークを使用した Rust 製 REST API サーバー

## アーキテクチャ

このプロジェクトはレイヤードアーキテクチャ（多層アーキテクチャ）を採用しています。

```
┌─────────────────────────────────────┐
│     Controller (Presentation)       │  HTTPリクエスト/レスポンス処理
├─────────────────────────────────────┤
│     Service (Application)           │  ビジネスロジック
├─────────────────────────────────────┤
│     Repository (Data Access)        │  データベース操作
├─────────────────────────────────────┤
│     Models (Domain)                 │  エンティティ定義
└─────────────────────────────────────┘
```

## ディレクトリ構成

```
roket-api/
├── src/
│   ├── main.rs                       # エントリーポイント
│   ├── controller/                   # プレゼンテーション層
│   │   ├── mod.rs
│   │   └── user.rs                   # ユーザー関連エンドポイント
│   ├── services/                     # アプリケーション層
│   │   ├── mod.rs
│   │   └── user_service.rs           # ユーザービジネスロジック
│   ├── repositories/                 # データアクセス層
│   │   ├── mod.rs
│   │   └── user_repository.rs        # ユーザーDB操作
│   └── modules/                      # ドメインモデル
│       ├── mod.rs
│       └── user.rs                   # Userエンティティ
├── .env                              # 環境変数
└── Cargo.toml                        # 依存関係
```

※:`mod.rs`は、ディレクトリをモジュールとして公開するファイル。

## 各層の責務

### 1. Controller（プレゼンテーション層）

- HTTP リクエストの受付
- リクエストパラメータのバリデーション
- レスポンスの整形と返却
- HTTP ステータスコードの管理

**例**: `src/controller/user.rs`

```rust
#[get("/users")]
pub async fn get_users(pool: &State<PgPool>) -> Result<Json<Vec<User>>, Status>
```

### 2. Service（アプリケーション層）

- ビジネスロジックの実装
- トランザクション管理
- エラーハンドリング
- 複数のリポジトリの調整

**例**: `src/services/user_service.rs`

```rust
pub async fn get_all_users(pool: &PgPool) -> Result<Vec<User>, String>
```

### 3. Repository（データアクセス層）

- データベースへの CRUD 操作
- SQL クエリの実行
- O/R マッピング

**例**: `src/repositories/user_repository.rs`

```rust
pub async fn find_all(pool: &PgPool) -> Result<Vec<User>, sqlx::Error>
```

### 4. Models（ドメインモデル）

- エンティティの定義
- データ構造の型定義
- シリアライズ/デシリアライズの設定

**例**: `src/modules/user.rs`

```rust
pub struct User {
    pub user_id: i32,
    pub user_name: String,
    pub email: String,
}
```

## データフロー

```
HTTP Request
    ↓
Controller (user.rs)
    ↓ Serviceを呼び出し
Service (user_service.rs)
    ↓ Repositoryを呼び出し
Repository (user_repository.rs)
    ↓ SQLクエリ実行
Database (PostgreSQL)
    ↓ 結果を返却
Repository
    ↓ エンティティに変換
Service
    ↓ ビジネスロジック適用
Controller
    ↓ JSON形式に整形
HTTP Response
```

## 技術スタック

- **フレームワーク**: Rocket 0.5.1
- **データベース**: PostgreSQL
- **ORM**: SQLx 0.8
- **非同期ランタイム**: Tokio 1.x
- **環境変数管理**: dotenvy 0.15

## セットアップ

### 2. 依存関係のインストール

```bash
cargo build
```

### 3. アプリケーションの起動

```bash
cargo run
```

サーバーは `http://localhost:8000` で起動します。

## API エンドポイント

### ルートエンドポイント

- `GET /` - ヘルスチェック

### ユーザーエンドポイント

- `GET /api/users` - 全ユーザー取得
- `POST /api/users` - ユーザー作成

## レイヤードアーキテクチャの利点

1. **関心の分離**: 各層が明確な責務を持つ
2. **テスタビリティ**: 各層を独立してテスト可能
3. **保守性**: 変更の影響範囲が限定される
4. **再利用性**: サービス層のロジックを複数のコントローラーから利用可能
5. **拡張性**: 新しい機能を追加しやすい

## 開発ガイドライン

### 新しいエンティティを追加する場合

1. `modules/` にモデルを定義
2. `repositories/` にデータアクセス層を実装
3. `services/` にビジネスロジックを実装
4. `controller/` にエンドポイントを追加
5. `main.rs` でルーティングを設定

### 命名規則

- **Controller**: `{entity}.rs` - 例: `user.rs`
- **Service**: `{entity}_service.rs` - 例: `user_service.rs`
- **Repository**: `{entity}_repository.rs` - 例: `user_repository.rs`
- **Model**: `{entity}.rs` - 例: `user.rs`
