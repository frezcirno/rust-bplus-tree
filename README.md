# B-Plus Tree Data Structure in Rust
- B+ 树是一种树数据结构，是一个N叉树，每个节点通常有多个孩子，一颗B+树包含根节点、内部节点和叶子节点。
- B+ 树通常用于数据库和操作系统的文件系统中。 B+树的特点是能够保持数据稳定有序，其插入与修改拥有较稳定的对数时间复杂度。 B+树元素自底向上插入。
## 小组成员（以姓氏排名）
**组长 谭梓煊** （frezcirno Zixuan Tan）
陈俊凯（iCSawyer Junkai Chen） 沈力瑜 吴佳琦（hunter-jacky）项建航（Pigeonwx）
## 项目文件结构
-- rust-bplus-tree  
&nbsp;&nbsp;&nbsp;&nbsp;|---src:项目主体源代码  
&nbsp;&nbsp;&nbsp;&nbsp;|---tests:项目测试程序  
&nbsp;&nbsp;&nbsp;&nbsp;|---Cargo.toml:内置的依赖管理器和构建工具，它能轻松增加、编译和管理依赖  
## B+ 树特征
一棵m阶的B+树或者为空树或者满足以下特性：
  1. 树中的每个节点最多有m个子树
  2. 若根节点不是叶子节点则至少有2个子树
  3. 除根之外的非叶子节点至少有[m/2]个子树
  4. 有 K 个子树的非叶节点恰好包含 K 个关键字
  5. 对于所有的非叶子结点，指针Pi-1所指子树中所有结点的关键字均小于Ki，Pn所指子树中所有结点的关键字均大于Kn
## 关键设计
### 节点类型
- **索引节点**：用于B+树的快速查找，分裂、合并。
```
pub struct BPIndexNode<const FANOUT: usize, K: Copy + Ord + Debug, V: Clone + Debug> {
    keys: Vec<K>,
    children: Vec<BPNodePtr<FANOUT, K, V>>,
    parent: Option<BPNodeWeak<FANOUT, K, V>>,
    pub prev: Option<BPNodeWeak<FANOUT, K, V>>,
    pub next: Option< BPNodeWeak<FANOUT, K, V>>,
}
```
- **叶子节点**：存储实际的保存在B+树中的值。
```
pub struct BPLeafNode<const FANOUT: usize, K: Copy + Ord + Debug, V: Clone + Debug> {
    keys: Vec<K>,
    values: Vec<V>,
    parent: Option<BPNodeWeak<FANOUT, K, V>>,
    pub prev: Option<BPNodeWeak<FANOUT, K, V>>,
    pub next: Option< BPNodeWeak<FANOUT, K, V>>,
}
```
- 抽象B+树节点
```
pub enum BPNode<const FANOUT: usize, K, V> {
    Index(BPIndexNode<FANOUT, K, V>),
    Leaf(BPLeafNode<FANOUT, K, V>),
}
```
### B+树操作
- 添加
  - pub fn insert(&mut self, key: K, value: V)  
    - BPTree调用insert方法插入元素，在其内部调用insert_recur执行实际的插入操作。
  - pub(crate) fn insert_recur(root: &BPNodePtr<FANOUT, K, V>, key: K, value: V)
    - 使用递归的方式插入元素，并在需要分裂节点的地方调用split_node的分裂出两个子节点

- 删除
  - pub fn remove(&mut self, key: &K)
    - BPTree调用remove方法删除元素，在其内部调用remove_recur执行实际的删除操作。
  - pub(crate) fn remove_recur(root: &BPNodePtr<FANOUT, K, V>, key: &K)
    - 使用递归的方式删除元素在某些条件下由于删除元素的特性会使B+树不符合其特征：1.删除后兄弟节点节点没有多余的关键字，则需要同其兄弟结点进行合并。调用 merge_children 。2.兄弟结点中含有多余的关键字，可以从兄弟结点中借关键字完成删除操作，调用 rebalance_children。

- 查找
  - pub(crate) fn search_recur(root: &BPNodePtr<FANOUT, K, V>, key: &K) -> Option<V>  
    - 使用不同节点类型进行递归查找，节点内的所有元素都以升序方式排序放置。

### B+树优化
**1. 为B+树节点实现了Debug trait特质，便于打印调试，B+树的可视化**
  - impl<const FANOUT: usize, K , V> Debug for BPIndexNode<FANOUT, K, V>
  - impl<const FANOUT: usize, K: Copy + Ord + Debug, V: Clone + Debug> Debug for BPLeafNode<FANOUT, K, V>  
  
**2. 为B+树进行了详尽的功能性单元测试，并发环境下的测试**
  - 详见项目下的相关测试代码
**3. 非叶子节点间加指针，模仿b*树做优化
  - 背景：检查后一个兄弟节点是否元素已满，充分利用树的相关空间
  - 做法：
    - 为IndexNode添加prev和next指针
    ```
    pub struct BPIndexNode<const FANOUT: usize, K: Copy + Ord + Debug, V: Clone + Debug> {
      keys: Vec<K>,
      children: Vec<BPNodePtr<FANOUT, K, V>>,
      parent: Option<BPNodeWeak<FANOUT, K, V>>,
      pub prev: Option<BPNodeWeak<FANOUT, K, V>>,
      pub next: Option<BPNodeWeak<FANOUT, K, V>>,
    }
    ```
    - 在元素插入过程中进行判断。如果有兄弟节点则对其空间进行利用，避免分配额外的节点空间。
## 心得体会
- 代码层面：
  1. 使用引用计数+内部可变性实现复杂的引用结构
  2. 可以通过递归代替迭代消除额外的clone()
- 团队层面：
  1. 在组长的带领下，团队成员分工明确，通过使用github进行协作开发，提高开发效率。
  2. 对于项目开发过程遇到的问题，及时在团队内提出，共同思考解决方法，尽早解决问题，减少损失。
- 课程层面：
  1. 通过项目开发的形式，加深对RUST语言的理解，提高实践能力。
  2. Rust 作为一门通用系统级编程语言，它可以有效的解决编程时引入内存访问问题、同时保持高性能，并具有出色的内存安全机制。
