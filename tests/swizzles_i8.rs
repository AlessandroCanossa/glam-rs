// Generated by swizzlegen. Do not edit.
#[macro_use]
mod support;
use glam::*;

glam_test!(test_i8vec4_swizzles, {
    let v = i8vec4(1_i8, 2_i8, 3_i8, 4_i8);
    assert_eq!(v, v.xyzw());
    assert_eq!(v.xxxx(), i8vec4(1_i8, 1_i8, 1_i8, 1_i8));
    assert_eq!(v.xxxy(), i8vec4(1_i8, 1_i8, 1_i8, 2_i8));
    assert_eq!(v.xxxz(), i8vec4(1_i8, 1_i8, 1_i8, 3_i8));
    assert_eq!(v.xxxw(), i8vec4(1_i8, 1_i8, 1_i8, 4_i8));
    assert_eq!(v.xxyx(), i8vec4(1_i8, 1_i8, 2_i8, 1_i8));
    assert_eq!(v.xxyy(), i8vec4(1_i8, 1_i8, 2_i8, 2_i8));
    assert_eq!(v.xxyz(), i8vec4(1_i8, 1_i8, 2_i8, 3_i8));
    assert_eq!(v.xxyw(), i8vec4(1_i8, 1_i8, 2_i8, 4_i8));
    assert_eq!(v.xxzx(), i8vec4(1_i8, 1_i8, 3_i8, 1_i8));
    assert_eq!(v.xxzy(), i8vec4(1_i8, 1_i8, 3_i8, 2_i8));
    assert_eq!(v.xxzz(), i8vec4(1_i8, 1_i8, 3_i8, 3_i8));
    assert_eq!(v.xxzw(), i8vec4(1_i8, 1_i8, 3_i8, 4_i8));
    assert_eq!(v.xxwx(), i8vec4(1_i8, 1_i8, 4_i8, 1_i8));
    assert_eq!(v.xxwy(), i8vec4(1_i8, 1_i8, 4_i8, 2_i8));
    assert_eq!(v.xxwz(), i8vec4(1_i8, 1_i8, 4_i8, 3_i8));
    assert_eq!(v.xxww(), i8vec4(1_i8, 1_i8, 4_i8, 4_i8));
    assert_eq!(v.xyxx(), i8vec4(1_i8, 2_i8, 1_i8, 1_i8));
    assert_eq!(v.xyxy(), i8vec4(1_i8, 2_i8, 1_i8, 2_i8));
    assert_eq!(v.xyxz(), i8vec4(1_i8, 2_i8, 1_i8, 3_i8));
    assert_eq!(v.xyxw(), i8vec4(1_i8, 2_i8, 1_i8, 4_i8));
    assert_eq!(v.xyyx(), i8vec4(1_i8, 2_i8, 2_i8, 1_i8));
    assert_eq!(v.xyyy(), i8vec4(1_i8, 2_i8, 2_i8, 2_i8));
    assert_eq!(v.xyyz(), i8vec4(1_i8, 2_i8, 2_i8, 3_i8));
    assert_eq!(v.xyyw(), i8vec4(1_i8, 2_i8, 2_i8, 4_i8));
    assert_eq!(v.xyzx(), i8vec4(1_i8, 2_i8, 3_i8, 1_i8));
    assert_eq!(v.xyzy(), i8vec4(1_i8, 2_i8, 3_i8, 2_i8));
    assert_eq!(v.xyzz(), i8vec4(1_i8, 2_i8, 3_i8, 3_i8));
    assert_eq!(v.xywx(), i8vec4(1_i8, 2_i8, 4_i8, 1_i8));
    assert_eq!(v.xywy(), i8vec4(1_i8, 2_i8, 4_i8, 2_i8));
    assert_eq!(v.xywz(), i8vec4(1_i8, 2_i8, 4_i8, 3_i8));
    assert_eq!(v.xyww(), i8vec4(1_i8, 2_i8, 4_i8, 4_i8));
    assert_eq!(v.xzxx(), i8vec4(1_i8, 3_i8, 1_i8, 1_i8));
    assert_eq!(v.xzxy(), i8vec4(1_i8, 3_i8, 1_i8, 2_i8));
    assert_eq!(v.xzxz(), i8vec4(1_i8, 3_i8, 1_i8, 3_i8));
    assert_eq!(v.xzxw(), i8vec4(1_i8, 3_i8, 1_i8, 4_i8));
    assert_eq!(v.xzyx(), i8vec4(1_i8, 3_i8, 2_i8, 1_i8));
    assert_eq!(v.xzyy(), i8vec4(1_i8, 3_i8, 2_i8, 2_i8));
    assert_eq!(v.xzyz(), i8vec4(1_i8, 3_i8, 2_i8, 3_i8));
    assert_eq!(v.xzyw(), i8vec4(1_i8, 3_i8, 2_i8, 4_i8));
    assert_eq!(v.xzzx(), i8vec4(1_i8, 3_i8, 3_i8, 1_i8));
    assert_eq!(v.xzzy(), i8vec4(1_i8, 3_i8, 3_i8, 2_i8));
    assert_eq!(v.xzzz(), i8vec4(1_i8, 3_i8, 3_i8, 3_i8));
    assert_eq!(v.xzzw(), i8vec4(1_i8, 3_i8, 3_i8, 4_i8));
    assert_eq!(v.xzwx(), i8vec4(1_i8, 3_i8, 4_i8, 1_i8));
    assert_eq!(v.xzwy(), i8vec4(1_i8, 3_i8, 4_i8, 2_i8));
    assert_eq!(v.xzwz(), i8vec4(1_i8, 3_i8, 4_i8, 3_i8));
    assert_eq!(v.xzww(), i8vec4(1_i8, 3_i8, 4_i8, 4_i8));
    assert_eq!(v.xwxx(), i8vec4(1_i8, 4_i8, 1_i8, 1_i8));
    assert_eq!(v.xwxy(), i8vec4(1_i8, 4_i8, 1_i8, 2_i8));
    assert_eq!(v.xwxz(), i8vec4(1_i8, 4_i8, 1_i8, 3_i8));
    assert_eq!(v.xwxw(), i8vec4(1_i8, 4_i8, 1_i8, 4_i8));
    assert_eq!(v.xwyx(), i8vec4(1_i8, 4_i8, 2_i8, 1_i8));
    assert_eq!(v.xwyy(), i8vec4(1_i8, 4_i8, 2_i8, 2_i8));
    assert_eq!(v.xwyz(), i8vec4(1_i8, 4_i8, 2_i8, 3_i8));
    assert_eq!(v.xwyw(), i8vec4(1_i8, 4_i8, 2_i8, 4_i8));
    assert_eq!(v.xwzx(), i8vec4(1_i8, 4_i8, 3_i8, 1_i8));
    assert_eq!(v.xwzy(), i8vec4(1_i8, 4_i8, 3_i8, 2_i8));
    assert_eq!(v.xwzz(), i8vec4(1_i8, 4_i8, 3_i8, 3_i8));
    assert_eq!(v.xwzw(), i8vec4(1_i8, 4_i8, 3_i8, 4_i8));
    assert_eq!(v.xwwx(), i8vec4(1_i8, 4_i8, 4_i8, 1_i8));
    assert_eq!(v.xwwy(), i8vec4(1_i8, 4_i8, 4_i8, 2_i8));
    assert_eq!(v.xwwz(), i8vec4(1_i8, 4_i8, 4_i8, 3_i8));
    assert_eq!(v.xwww(), i8vec4(1_i8, 4_i8, 4_i8, 4_i8));
    assert_eq!(v.yxxx(), i8vec4(2_i8, 1_i8, 1_i8, 1_i8));
    assert_eq!(v.yxxy(), i8vec4(2_i8, 1_i8, 1_i8, 2_i8));
    assert_eq!(v.yxxz(), i8vec4(2_i8, 1_i8, 1_i8, 3_i8));
    assert_eq!(v.yxxw(), i8vec4(2_i8, 1_i8, 1_i8, 4_i8));
    assert_eq!(v.yxyx(), i8vec4(2_i8, 1_i8, 2_i8, 1_i8));
    assert_eq!(v.yxyy(), i8vec4(2_i8, 1_i8, 2_i8, 2_i8));
    assert_eq!(v.yxyz(), i8vec4(2_i8, 1_i8, 2_i8, 3_i8));
    assert_eq!(v.yxyw(), i8vec4(2_i8, 1_i8, 2_i8, 4_i8));
    assert_eq!(v.yxzx(), i8vec4(2_i8, 1_i8, 3_i8, 1_i8));
    assert_eq!(v.yxzy(), i8vec4(2_i8, 1_i8, 3_i8, 2_i8));
    assert_eq!(v.yxzz(), i8vec4(2_i8, 1_i8, 3_i8, 3_i8));
    assert_eq!(v.yxzw(), i8vec4(2_i8, 1_i8, 3_i8, 4_i8));
    assert_eq!(v.yxwx(), i8vec4(2_i8, 1_i8, 4_i8, 1_i8));
    assert_eq!(v.yxwy(), i8vec4(2_i8, 1_i8, 4_i8, 2_i8));
    assert_eq!(v.yxwz(), i8vec4(2_i8, 1_i8, 4_i8, 3_i8));
    assert_eq!(v.yxww(), i8vec4(2_i8, 1_i8, 4_i8, 4_i8));
    assert_eq!(v.yyxx(), i8vec4(2_i8, 2_i8, 1_i8, 1_i8));
    assert_eq!(v.yyxy(), i8vec4(2_i8, 2_i8, 1_i8, 2_i8));
    assert_eq!(v.yyxz(), i8vec4(2_i8, 2_i8, 1_i8, 3_i8));
    assert_eq!(v.yyxw(), i8vec4(2_i8, 2_i8, 1_i8, 4_i8));
    assert_eq!(v.yyyx(), i8vec4(2_i8, 2_i8, 2_i8, 1_i8));
    assert_eq!(v.yyyy(), i8vec4(2_i8, 2_i8, 2_i8, 2_i8));
    assert_eq!(v.yyyz(), i8vec4(2_i8, 2_i8, 2_i8, 3_i8));
    assert_eq!(v.yyyw(), i8vec4(2_i8, 2_i8, 2_i8, 4_i8));
    assert_eq!(v.yyzx(), i8vec4(2_i8, 2_i8, 3_i8, 1_i8));
    assert_eq!(v.yyzy(), i8vec4(2_i8, 2_i8, 3_i8, 2_i8));
    assert_eq!(v.yyzz(), i8vec4(2_i8, 2_i8, 3_i8, 3_i8));
    assert_eq!(v.yyzw(), i8vec4(2_i8, 2_i8, 3_i8, 4_i8));
    assert_eq!(v.yywx(), i8vec4(2_i8, 2_i8, 4_i8, 1_i8));
    assert_eq!(v.yywy(), i8vec4(2_i8, 2_i8, 4_i8, 2_i8));
    assert_eq!(v.yywz(), i8vec4(2_i8, 2_i8, 4_i8, 3_i8));
    assert_eq!(v.yyww(), i8vec4(2_i8, 2_i8, 4_i8, 4_i8));
    assert_eq!(v.yzxx(), i8vec4(2_i8, 3_i8, 1_i8, 1_i8));
    assert_eq!(v.yzxy(), i8vec4(2_i8, 3_i8, 1_i8, 2_i8));
    assert_eq!(v.yzxz(), i8vec4(2_i8, 3_i8, 1_i8, 3_i8));
    assert_eq!(v.yzxw(), i8vec4(2_i8, 3_i8, 1_i8, 4_i8));
    assert_eq!(v.yzyx(), i8vec4(2_i8, 3_i8, 2_i8, 1_i8));
    assert_eq!(v.yzyy(), i8vec4(2_i8, 3_i8, 2_i8, 2_i8));
    assert_eq!(v.yzyz(), i8vec4(2_i8, 3_i8, 2_i8, 3_i8));
    assert_eq!(v.yzyw(), i8vec4(2_i8, 3_i8, 2_i8, 4_i8));
    assert_eq!(v.yzzx(), i8vec4(2_i8, 3_i8, 3_i8, 1_i8));
    assert_eq!(v.yzzy(), i8vec4(2_i8, 3_i8, 3_i8, 2_i8));
    assert_eq!(v.yzzz(), i8vec4(2_i8, 3_i8, 3_i8, 3_i8));
    assert_eq!(v.yzzw(), i8vec4(2_i8, 3_i8, 3_i8, 4_i8));
    assert_eq!(v.yzwx(), i8vec4(2_i8, 3_i8, 4_i8, 1_i8));
    assert_eq!(v.yzwy(), i8vec4(2_i8, 3_i8, 4_i8, 2_i8));
    assert_eq!(v.yzwz(), i8vec4(2_i8, 3_i8, 4_i8, 3_i8));
    assert_eq!(v.yzww(), i8vec4(2_i8, 3_i8, 4_i8, 4_i8));
    assert_eq!(v.ywxx(), i8vec4(2_i8, 4_i8, 1_i8, 1_i8));
    assert_eq!(v.ywxy(), i8vec4(2_i8, 4_i8, 1_i8, 2_i8));
    assert_eq!(v.ywxz(), i8vec4(2_i8, 4_i8, 1_i8, 3_i8));
    assert_eq!(v.ywxw(), i8vec4(2_i8, 4_i8, 1_i8, 4_i8));
    assert_eq!(v.ywyx(), i8vec4(2_i8, 4_i8, 2_i8, 1_i8));
    assert_eq!(v.ywyy(), i8vec4(2_i8, 4_i8, 2_i8, 2_i8));
    assert_eq!(v.ywyz(), i8vec4(2_i8, 4_i8, 2_i8, 3_i8));
    assert_eq!(v.ywyw(), i8vec4(2_i8, 4_i8, 2_i8, 4_i8));
    assert_eq!(v.ywzx(), i8vec4(2_i8, 4_i8, 3_i8, 1_i8));
    assert_eq!(v.ywzy(), i8vec4(2_i8, 4_i8, 3_i8, 2_i8));
    assert_eq!(v.ywzz(), i8vec4(2_i8, 4_i8, 3_i8, 3_i8));
    assert_eq!(v.ywzw(), i8vec4(2_i8, 4_i8, 3_i8, 4_i8));
    assert_eq!(v.ywwx(), i8vec4(2_i8, 4_i8, 4_i8, 1_i8));
    assert_eq!(v.ywwy(), i8vec4(2_i8, 4_i8, 4_i8, 2_i8));
    assert_eq!(v.ywwz(), i8vec4(2_i8, 4_i8, 4_i8, 3_i8));
    assert_eq!(v.ywww(), i8vec4(2_i8, 4_i8, 4_i8, 4_i8));
    assert_eq!(v.zxxx(), i8vec4(3_i8, 1_i8, 1_i8, 1_i8));
    assert_eq!(v.zxxy(), i8vec4(3_i8, 1_i8, 1_i8, 2_i8));
    assert_eq!(v.zxxz(), i8vec4(3_i8, 1_i8, 1_i8, 3_i8));
    assert_eq!(v.zxxw(), i8vec4(3_i8, 1_i8, 1_i8, 4_i8));
    assert_eq!(v.zxyx(), i8vec4(3_i8, 1_i8, 2_i8, 1_i8));
    assert_eq!(v.zxyy(), i8vec4(3_i8, 1_i8, 2_i8, 2_i8));
    assert_eq!(v.zxyz(), i8vec4(3_i8, 1_i8, 2_i8, 3_i8));
    assert_eq!(v.zxyw(), i8vec4(3_i8, 1_i8, 2_i8, 4_i8));
    assert_eq!(v.zxzx(), i8vec4(3_i8, 1_i8, 3_i8, 1_i8));
    assert_eq!(v.zxzy(), i8vec4(3_i8, 1_i8, 3_i8, 2_i8));
    assert_eq!(v.zxzz(), i8vec4(3_i8, 1_i8, 3_i8, 3_i8));
    assert_eq!(v.zxzw(), i8vec4(3_i8, 1_i8, 3_i8, 4_i8));
    assert_eq!(v.zxwx(), i8vec4(3_i8, 1_i8, 4_i8, 1_i8));
    assert_eq!(v.zxwy(), i8vec4(3_i8, 1_i8, 4_i8, 2_i8));
    assert_eq!(v.zxwz(), i8vec4(3_i8, 1_i8, 4_i8, 3_i8));
    assert_eq!(v.zxww(), i8vec4(3_i8, 1_i8, 4_i8, 4_i8));
    assert_eq!(v.zyxx(), i8vec4(3_i8, 2_i8, 1_i8, 1_i8));
    assert_eq!(v.zyxy(), i8vec4(3_i8, 2_i8, 1_i8, 2_i8));
    assert_eq!(v.zyxz(), i8vec4(3_i8, 2_i8, 1_i8, 3_i8));
    assert_eq!(v.zyxw(), i8vec4(3_i8, 2_i8, 1_i8, 4_i8));
    assert_eq!(v.zyyx(), i8vec4(3_i8, 2_i8, 2_i8, 1_i8));
    assert_eq!(v.zyyy(), i8vec4(3_i8, 2_i8, 2_i8, 2_i8));
    assert_eq!(v.zyyz(), i8vec4(3_i8, 2_i8, 2_i8, 3_i8));
    assert_eq!(v.zyyw(), i8vec4(3_i8, 2_i8, 2_i8, 4_i8));
    assert_eq!(v.zyzx(), i8vec4(3_i8, 2_i8, 3_i8, 1_i8));
    assert_eq!(v.zyzy(), i8vec4(3_i8, 2_i8, 3_i8, 2_i8));
    assert_eq!(v.zyzz(), i8vec4(3_i8, 2_i8, 3_i8, 3_i8));
    assert_eq!(v.zyzw(), i8vec4(3_i8, 2_i8, 3_i8, 4_i8));
    assert_eq!(v.zywx(), i8vec4(3_i8, 2_i8, 4_i8, 1_i8));
    assert_eq!(v.zywy(), i8vec4(3_i8, 2_i8, 4_i8, 2_i8));
    assert_eq!(v.zywz(), i8vec4(3_i8, 2_i8, 4_i8, 3_i8));
    assert_eq!(v.zyww(), i8vec4(3_i8, 2_i8, 4_i8, 4_i8));
    assert_eq!(v.zzxx(), i8vec4(3_i8, 3_i8, 1_i8, 1_i8));
    assert_eq!(v.zzxy(), i8vec4(3_i8, 3_i8, 1_i8, 2_i8));
    assert_eq!(v.zzxz(), i8vec4(3_i8, 3_i8, 1_i8, 3_i8));
    assert_eq!(v.zzxw(), i8vec4(3_i8, 3_i8, 1_i8, 4_i8));
    assert_eq!(v.zzyx(), i8vec4(3_i8, 3_i8, 2_i8, 1_i8));
    assert_eq!(v.zzyy(), i8vec4(3_i8, 3_i8, 2_i8, 2_i8));
    assert_eq!(v.zzyz(), i8vec4(3_i8, 3_i8, 2_i8, 3_i8));
    assert_eq!(v.zzyw(), i8vec4(3_i8, 3_i8, 2_i8, 4_i8));
    assert_eq!(v.zzzx(), i8vec4(3_i8, 3_i8, 3_i8, 1_i8));
    assert_eq!(v.zzzy(), i8vec4(3_i8, 3_i8, 3_i8, 2_i8));
    assert_eq!(v.zzzz(), i8vec4(3_i8, 3_i8, 3_i8, 3_i8));
    assert_eq!(v.zzzw(), i8vec4(3_i8, 3_i8, 3_i8, 4_i8));
    assert_eq!(v.zzwx(), i8vec4(3_i8, 3_i8, 4_i8, 1_i8));
    assert_eq!(v.zzwy(), i8vec4(3_i8, 3_i8, 4_i8, 2_i8));
    assert_eq!(v.zzwz(), i8vec4(3_i8, 3_i8, 4_i8, 3_i8));
    assert_eq!(v.zzww(), i8vec4(3_i8, 3_i8, 4_i8, 4_i8));
    assert_eq!(v.zwxx(), i8vec4(3_i8, 4_i8, 1_i8, 1_i8));
    assert_eq!(v.zwxy(), i8vec4(3_i8, 4_i8, 1_i8, 2_i8));
    assert_eq!(v.zwxz(), i8vec4(3_i8, 4_i8, 1_i8, 3_i8));
    assert_eq!(v.zwxw(), i8vec4(3_i8, 4_i8, 1_i8, 4_i8));
    assert_eq!(v.zwyx(), i8vec4(3_i8, 4_i8, 2_i8, 1_i8));
    assert_eq!(v.zwyy(), i8vec4(3_i8, 4_i8, 2_i8, 2_i8));
    assert_eq!(v.zwyz(), i8vec4(3_i8, 4_i8, 2_i8, 3_i8));
    assert_eq!(v.zwyw(), i8vec4(3_i8, 4_i8, 2_i8, 4_i8));
    assert_eq!(v.zwzx(), i8vec4(3_i8, 4_i8, 3_i8, 1_i8));
    assert_eq!(v.zwzy(), i8vec4(3_i8, 4_i8, 3_i8, 2_i8));
    assert_eq!(v.zwzz(), i8vec4(3_i8, 4_i8, 3_i8, 3_i8));
    assert_eq!(v.zwzw(), i8vec4(3_i8, 4_i8, 3_i8, 4_i8));
    assert_eq!(v.zwwx(), i8vec4(3_i8, 4_i8, 4_i8, 1_i8));
    assert_eq!(v.zwwy(), i8vec4(3_i8, 4_i8, 4_i8, 2_i8));
    assert_eq!(v.zwwz(), i8vec4(3_i8, 4_i8, 4_i8, 3_i8));
    assert_eq!(v.zwww(), i8vec4(3_i8, 4_i8, 4_i8, 4_i8));
    assert_eq!(v.wxxx(), i8vec4(4_i8, 1_i8, 1_i8, 1_i8));
    assert_eq!(v.wxxy(), i8vec4(4_i8, 1_i8, 1_i8, 2_i8));
    assert_eq!(v.wxxz(), i8vec4(4_i8, 1_i8, 1_i8, 3_i8));
    assert_eq!(v.wxxw(), i8vec4(4_i8, 1_i8, 1_i8, 4_i8));
    assert_eq!(v.wxyx(), i8vec4(4_i8, 1_i8, 2_i8, 1_i8));
    assert_eq!(v.wxyy(), i8vec4(4_i8, 1_i8, 2_i8, 2_i8));
    assert_eq!(v.wxyz(), i8vec4(4_i8, 1_i8, 2_i8, 3_i8));
    assert_eq!(v.wxyw(), i8vec4(4_i8, 1_i8, 2_i8, 4_i8));
    assert_eq!(v.wxzx(), i8vec4(4_i8, 1_i8, 3_i8, 1_i8));
    assert_eq!(v.wxzy(), i8vec4(4_i8, 1_i8, 3_i8, 2_i8));
    assert_eq!(v.wxzz(), i8vec4(4_i8, 1_i8, 3_i8, 3_i8));
    assert_eq!(v.wxzw(), i8vec4(4_i8, 1_i8, 3_i8, 4_i8));
    assert_eq!(v.wxwx(), i8vec4(4_i8, 1_i8, 4_i8, 1_i8));
    assert_eq!(v.wxwy(), i8vec4(4_i8, 1_i8, 4_i8, 2_i8));
    assert_eq!(v.wxwz(), i8vec4(4_i8, 1_i8, 4_i8, 3_i8));
    assert_eq!(v.wxww(), i8vec4(4_i8, 1_i8, 4_i8, 4_i8));
    assert_eq!(v.wyxx(), i8vec4(4_i8, 2_i8, 1_i8, 1_i8));
    assert_eq!(v.wyxy(), i8vec4(4_i8, 2_i8, 1_i8, 2_i8));
    assert_eq!(v.wyxz(), i8vec4(4_i8, 2_i8, 1_i8, 3_i8));
    assert_eq!(v.wyxw(), i8vec4(4_i8, 2_i8, 1_i8, 4_i8));
    assert_eq!(v.wyyx(), i8vec4(4_i8, 2_i8, 2_i8, 1_i8));
    assert_eq!(v.wyyy(), i8vec4(4_i8, 2_i8, 2_i8, 2_i8));
    assert_eq!(v.wyyz(), i8vec4(4_i8, 2_i8, 2_i8, 3_i8));
    assert_eq!(v.wyyw(), i8vec4(4_i8, 2_i8, 2_i8, 4_i8));
    assert_eq!(v.wyzx(), i8vec4(4_i8, 2_i8, 3_i8, 1_i8));
    assert_eq!(v.wyzy(), i8vec4(4_i8, 2_i8, 3_i8, 2_i8));
    assert_eq!(v.wyzz(), i8vec4(4_i8, 2_i8, 3_i8, 3_i8));
    assert_eq!(v.wyzw(), i8vec4(4_i8, 2_i8, 3_i8, 4_i8));
    assert_eq!(v.wywx(), i8vec4(4_i8, 2_i8, 4_i8, 1_i8));
    assert_eq!(v.wywy(), i8vec4(4_i8, 2_i8, 4_i8, 2_i8));
    assert_eq!(v.wywz(), i8vec4(4_i8, 2_i8, 4_i8, 3_i8));
    assert_eq!(v.wyww(), i8vec4(4_i8, 2_i8, 4_i8, 4_i8));
    assert_eq!(v.wzxx(), i8vec4(4_i8, 3_i8, 1_i8, 1_i8));
    assert_eq!(v.wzxy(), i8vec4(4_i8, 3_i8, 1_i8, 2_i8));
    assert_eq!(v.wzxz(), i8vec4(4_i8, 3_i8, 1_i8, 3_i8));
    assert_eq!(v.wzxw(), i8vec4(4_i8, 3_i8, 1_i8, 4_i8));
    assert_eq!(v.wzyx(), i8vec4(4_i8, 3_i8, 2_i8, 1_i8));
    assert_eq!(v.wzyy(), i8vec4(4_i8, 3_i8, 2_i8, 2_i8));
    assert_eq!(v.wzyz(), i8vec4(4_i8, 3_i8, 2_i8, 3_i8));
    assert_eq!(v.wzyw(), i8vec4(4_i8, 3_i8, 2_i8, 4_i8));
    assert_eq!(v.wzzx(), i8vec4(4_i8, 3_i8, 3_i8, 1_i8));
    assert_eq!(v.wzzy(), i8vec4(4_i8, 3_i8, 3_i8, 2_i8));
    assert_eq!(v.wzzz(), i8vec4(4_i8, 3_i8, 3_i8, 3_i8));
    assert_eq!(v.wzzw(), i8vec4(4_i8, 3_i8, 3_i8, 4_i8));
    assert_eq!(v.wzwx(), i8vec4(4_i8, 3_i8, 4_i8, 1_i8));
    assert_eq!(v.wzwy(), i8vec4(4_i8, 3_i8, 4_i8, 2_i8));
    assert_eq!(v.wzwz(), i8vec4(4_i8, 3_i8, 4_i8, 3_i8));
    assert_eq!(v.wzww(), i8vec4(4_i8, 3_i8, 4_i8, 4_i8));
    assert_eq!(v.wwxx(), i8vec4(4_i8, 4_i8, 1_i8, 1_i8));
    assert_eq!(v.wwxy(), i8vec4(4_i8, 4_i8, 1_i8, 2_i8));
    assert_eq!(v.wwxz(), i8vec4(4_i8, 4_i8, 1_i8, 3_i8));
    assert_eq!(v.wwxw(), i8vec4(4_i8, 4_i8, 1_i8, 4_i8));
    assert_eq!(v.wwyx(), i8vec4(4_i8, 4_i8, 2_i8, 1_i8));
    assert_eq!(v.wwyy(), i8vec4(4_i8, 4_i8, 2_i8, 2_i8));
    assert_eq!(v.wwyz(), i8vec4(4_i8, 4_i8, 2_i8, 3_i8));
    assert_eq!(v.wwyw(), i8vec4(4_i8, 4_i8, 2_i8, 4_i8));
    assert_eq!(v.wwzx(), i8vec4(4_i8, 4_i8, 3_i8, 1_i8));
    assert_eq!(v.wwzy(), i8vec4(4_i8, 4_i8, 3_i8, 2_i8));
    assert_eq!(v.wwzz(), i8vec4(4_i8, 4_i8, 3_i8, 3_i8));
    assert_eq!(v.wwzw(), i8vec4(4_i8, 4_i8, 3_i8, 4_i8));
    assert_eq!(v.wwwx(), i8vec4(4_i8, 4_i8, 4_i8, 1_i8));
    assert_eq!(v.wwwy(), i8vec4(4_i8, 4_i8, 4_i8, 2_i8));
    assert_eq!(v.wwwz(), i8vec4(4_i8, 4_i8, 4_i8, 3_i8));
    assert_eq!(v.wwww(), i8vec4(4_i8, 4_i8, 4_i8, 4_i8));
    assert_eq!(v.xxx(), i8vec3(1_i8, 1_i8, 1_i8));
    assert_eq!(v.xxy(), i8vec3(1_i8, 1_i8, 2_i8));
    assert_eq!(v.xxz(), i8vec3(1_i8, 1_i8, 3_i8));
    assert_eq!(v.xxw(), i8vec3(1_i8, 1_i8, 4_i8));
    assert_eq!(v.xyx(), i8vec3(1_i8, 2_i8, 1_i8));
    assert_eq!(v.xyy(), i8vec3(1_i8, 2_i8, 2_i8));
    assert_eq!(v.xyz(), i8vec3(1_i8, 2_i8, 3_i8));
    assert_eq!(v.xyw(), i8vec3(1_i8, 2_i8, 4_i8));
    assert_eq!(v.xzx(), i8vec3(1_i8, 3_i8, 1_i8));
    assert_eq!(v.xzy(), i8vec3(1_i8, 3_i8, 2_i8));
    assert_eq!(v.xzz(), i8vec3(1_i8, 3_i8, 3_i8));
    assert_eq!(v.xzw(), i8vec3(1_i8, 3_i8, 4_i8));
    assert_eq!(v.xwx(), i8vec3(1_i8, 4_i8, 1_i8));
    assert_eq!(v.xwy(), i8vec3(1_i8, 4_i8, 2_i8));
    assert_eq!(v.xwz(), i8vec3(1_i8, 4_i8, 3_i8));
    assert_eq!(v.xww(), i8vec3(1_i8, 4_i8, 4_i8));
    assert_eq!(v.yxx(), i8vec3(2_i8, 1_i8, 1_i8));
    assert_eq!(v.yxy(), i8vec3(2_i8, 1_i8, 2_i8));
    assert_eq!(v.yxz(), i8vec3(2_i8, 1_i8, 3_i8));
    assert_eq!(v.yxw(), i8vec3(2_i8, 1_i8, 4_i8));
    assert_eq!(v.yyx(), i8vec3(2_i8, 2_i8, 1_i8));
    assert_eq!(v.yyy(), i8vec3(2_i8, 2_i8, 2_i8));
    assert_eq!(v.yyz(), i8vec3(2_i8, 2_i8, 3_i8));
    assert_eq!(v.yyw(), i8vec3(2_i8, 2_i8, 4_i8));
    assert_eq!(v.yzx(), i8vec3(2_i8, 3_i8, 1_i8));
    assert_eq!(v.yzy(), i8vec3(2_i8, 3_i8, 2_i8));
    assert_eq!(v.yzz(), i8vec3(2_i8, 3_i8, 3_i8));
    assert_eq!(v.yzw(), i8vec3(2_i8, 3_i8, 4_i8));
    assert_eq!(v.ywx(), i8vec3(2_i8, 4_i8, 1_i8));
    assert_eq!(v.ywy(), i8vec3(2_i8, 4_i8, 2_i8));
    assert_eq!(v.ywz(), i8vec3(2_i8, 4_i8, 3_i8));
    assert_eq!(v.yww(), i8vec3(2_i8, 4_i8, 4_i8));
    assert_eq!(v.zxx(), i8vec3(3_i8, 1_i8, 1_i8));
    assert_eq!(v.zxy(), i8vec3(3_i8, 1_i8, 2_i8));
    assert_eq!(v.zxz(), i8vec3(3_i8, 1_i8, 3_i8));
    assert_eq!(v.zxw(), i8vec3(3_i8, 1_i8, 4_i8));
    assert_eq!(v.zyx(), i8vec3(3_i8, 2_i8, 1_i8));
    assert_eq!(v.zyy(), i8vec3(3_i8, 2_i8, 2_i8));
    assert_eq!(v.zyz(), i8vec3(3_i8, 2_i8, 3_i8));
    assert_eq!(v.zyw(), i8vec3(3_i8, 2_i8, 4_i8));
    assert_eq!(v.zzx(), i8vec3(3_i8, 3_i8, 1_i8));
    assert_eq!(v.zzy(), i8vec3(3_i8, 3_i8, 2_i8));
    assert_eq!(v.zzz(), i8vec3(3_i8, 3_i8, 3_i8));
    assert_eq!(v.zzw(), i8vec3(3_i8, 3_i8, 4_i8));
    assert_eq!(v.zwx(), i8vec3(3_i8, 4_i8, 1_i8));
    assert_eq!(v.zwy(), i8vec3(3_i8, 4_i8, 2_i8));
    assert_eq!(v.zwz(), i8vec3(3_i8, 4_i8, 3_i8));
    assert_eq!(v.zww(), i8vec3(3_i8, 4_i8, 4_i8));
    assert_eq!(v.wxx(), i8vec3(4_i8, 1_i8, 1_i8));
    assert_eq!(v.wxy(), i8vec3(4_i8, 1_i8, 2_i8));
    assert_eq!(v.wxz(), i8vec3(4_i8, 1_i8, 3_i8));
    assert_eq!(v.wxw(), i8vec3(4_i8, 1_i8, 4_i8));
    assert_eq!(v.wyx(), i8vec3(4_i8, 2_i8, 1_i8));
    assert_eq!(v.wyy(), i8vec3(4_i8, 2_i8, 2_i8));
    assert_eq!(v.wyz(), i8vec3(4_i8, 2_i8, 3_i8));
    assert_eq!(v.wyw(), i8vec3(4_i8, 2_i8, 4_i8));
    assert_eq!(v.wzx(), i8vec3(4_i8, 3_i8, 1_i8));
    assert_eq!(v.wzy(), i8vec3(4_i8, 3_i8, 2_i8));
    assert_eq!(v.wzz(), i8vec3(4_i8, 3_i8, 3_i8));
    assert_eq!(v.wzw(), i8vec3(4_i8, 3_i8, 4_i8));
    assert_eq!(v.wwx(), i8vec3(4_i8, 4_i8, 1_i8));
    assert_eq!(v.wwy(), i8vec3(4_i8, 4_i8, 2_i8));
    assert_eq!(v.wwz(), i8vec3(4_i8, 4_i8, 3_i8));
    assert_eq!(v.www(), i8vec3(4_i8, 4_i8, 4_i8));
    assert_eq!(v.xx(), i8vec2(1_i8, 1_i8));
    assert_eq!(v.xy(), i8vec2(1_i8, 2_i8));
    assert_eq!(v.xz(), i8vec2(1_i8, 3_i8));
    assert_eq!(v.xw(), i8vec2(1_i8, 4_i8));
    assert_eq!(v.yx(), i8vec2(2_i8, 1_i8));
    assert_eq!(v.yy(), i8vec2(2_i8, 2_i8));
    assert_eq!(v.yz(), i8vec2(2_i8, 3_i8));
    assert_eq!(v.yw(), i8vec2(2_i8, 4_i8));
    assert_eq!(v.zx(), i8vec2(3_i8, 1_i8));
    assert_eq!(v.zy(), i8vec2(3_i8, 2_i8));
    assert_eq!(v.zz(), i8vec2(3_i8, 3_i8));
    assert_eq!(v.zw(), i8vec2(3_i8, 4_i8));
    assert_eq!(v.wx(), i8vec2(4_i8, 1_i8));
    assert_eq!(v.wy(), i8vec2(4_i8, 2_i8));
    assert_eq!(v.wz(), i8vec2(4_i8, 3_i8));
    assert_eq!(v.ww(), i8vec2(4_i8, 4_i8));
});

glam_test!(test_i8vec3_swizzles, {
    let v = i8vec3(1_i8, 2_i8, 3_i8);
    assert_eq!(v, v.xyz());
    assert_eq!(v.xxxx(), i8vec4(1_i8, 1_i8, 1_i8, 1_i8));
    assert_eq!(v.xxxy(), i8vec4(1_i8, 1_i8, 1_i8, 2_i8));
    assert_eq!(v.xxxz(), i8vec4(1_i8, 1_i8, 1_i8, 3_i8));
    assert_eq!(v.xxyx(), i8vec4(1_i8, 1_i8, 2_i8, 1_i8));
    assert_eq!(v.xxyy(), i8vec4(1_i8, 1_i8, 2_i8, 2_i8));
    assert_eq!(v.xxyz(), i8vec4(1_i8, 1_i8, 2_i8, 3_i8));
    assert_eq!(v.xxzx(), i8vec4(1_i8, 1_i8, 3_i8, 1_i8));
    assert_eq!(v.xxzy(), i8vec4(1_i8, 1_i8, 3_i8, 2_i8));
    assert_eq!(v.xxzz(), i8vec4(1_i8, 1_i8, 3_i8, 3_i8));
    assert_eq!(v.xyxx(), i8vec4(1_i8, 2_i8, 1_i8, 1_i8));
    assert_eq!(v.xyxy(), i8vec4(1_i8, 2_i8, 1_i8, 2_i8));
    assert_eq!(v.xyxz(), i8vec4(1_i8, 2_i8, 1_i8, 3_i8));
    assert_eq!(v.xyyx(), i8vec4(1_i8, 2_i8, 2_i8, 1_i8));
    assert_eq!(v.xyyy(), i8vec4(1_i8, 2_i8, 2_i8, 2_i8));
    assert_eq!(v.xyyz(), i8vec4(1_i8, 2_i8, 2_i8, 3_i8));
    assert_eq!(v.xyzx(), i8vec4(1_i8, 2_i8, 3_i8, 1_i8));
    assert_eq!(v.xyzy(), i8vec4(1_i8, 2_i8, 3_i8, 2_i8));
    assert_eq!(v.xyzz(), i8vec4(1_i8, 2_i8, 3_i8, 3_i8));
    assert_eq!(v.xzxx(), i8vec4(1_i8, 3_i8, 1_i8, 1_i8));
    assert_eq!(v.xzxy(), i8vec4(1_i8, 3_i8, 1_i8, 2_i8));
    assert_eq!(v.xzxz(), i8vec4(1_i8, 3_i8, 1_i8, 3_i8));
    assert_eq!(v.xzyx(), i8vec4(1_i8, 3_i8, 2_i8, 1_i8));
    assert_eq!(v.xzyy(), i8vec4(1_i8, 3_i8, 2_i8, 2_i8));
    assert_eq!(v.xzyz(), i8vec4(1_i8, 3_i8, 2_i8, 3_i8));
    assert_eq!(v.xzzx(), i8vec4(1_i8, 3_i8, 3_i8, 1_i8));
    assert_eq!(v.xzzy(), i8vec4(1_i8, 3_i8, 3_i8, 2_i8));
    assert_eq!(v.xzzz(), i8vec4(1_i8, 3_i8, 3_i8, 3_i8));
    assert_eq!(v.yxxx(), i8vec4(2_i8, 1_i8, 1_i8, 1_i8));
    assert_eq!(v.yxxy(), i8vec4(2_i8, 1_i8, 1_i8, 2_i8));
    assert_eq!(v.yxxz(), i8vec4(2_i8, 1_i8, 1_i8, 3_i8));
    assert_eq!(v.yxyx(), i8vec4(2_i8, 1_i8, 2_i8, 1_i8));
    assert_eq!(v.yxyy(), i8vec4(2_i8, 1_i8, 2_i8, 2_i8));
    assert_eq!(v.yxyz(), i8vec4(2_i8, 1_i8, 2_i8, 3_i8));
    assert_eq!(v.yxzx(), i8vec4(2_i8, 1_i8, 3_i8, 1_i8));
    assert_eq!(v.yxzy(), i8vec4(2_i8, 1_i8, 3_i8, 2_i8));
    assert_eq!(v.yxzz(), i8vec4(2_i8, 1_i8, 3_i8, 3_i8));
    assert_eq!(v.yyxx(), i8vec4(2_i8, 2_i8, 1_i8, 1_i8));
    assert_eq!(v.yyxy(), i8vec4(2_i8, 2_i8, 1_i8, 2_i8));
    assert_eq!(v.yyxz(), i8vec4(2_i8, 2_i8, 1_i8, 3_i8));
    assert_eq!(v.yyyx(), i8vec4(2_i8, 2_i8, 2_i8, 1_i8));
    assert_eq!(v.yyyy(), i8vec4(2_i8, 2_i8, 2_i8, 2_i8));
    assert_eq!(v.yyyz(), i8vec4(2_i8, 2_i8, 2_i8, 3_i8));
    assert_eq!(v.yyzx(), i8vec4(2_i8, 2_i8, 3_i8, 1_i8));
    assert_eq!(v.yyzy(), i8vec4(2_i8, 2_i8, 3_i8, 2_i8));
    assert_eq!(v.yyzz(), i8vec4(2_i8, 2_i8, 3_i8, 3_i8));
    assert_eq!(v.yzxx(), i8vec4(2_i8, 3_i8, 1_i8, 1_i8));
    assert_eq!(v.yzxy(), i8vec4(2_i8, 3_i8, 1_i8, 2_i8));
    assert_eq!(v.yzxz(), i8vec4(2_i8, 3_i8, 1_i8, 3_i8));
    assert_eq!(v.yzyx(), i8vec4(2_i8, 3_i8, 2_i8, 1_i8));
    assert_eq!(v.yzyy(), i8vec4(2_i8, 3_i8, 2_i8, 2_i8));
    assert_eq!(v.yzyz(), i8vec4(2_i8, 3_i8, 2_i8, 3_i8));
    assert_eq!(v.yzzx(), i8vec4(2_i8, 3_i8, 3_i8, 1_i8));
    assert_eq!(v.yzzy(), i8vec4(2_i8, 3_i8, 3_i8, 2_i8));
    assert_eq!(v.yzzz(), i8vec4(2_i8, 3_i8, 3_i8, 3_i8));
    assert_eq!(v.zxxx(), i8vec4(3_i8, 1_i8, 1_i8, 1_i8));
    assert_eq!(v.zxxy(), i8vec4(3_i8, 1_i8, 1_i8, 2_i8));
    assert_eq!(v.zxxz(), i8vec4(3_i8, 1_i8, 1_i8, 3_i8));
    assert_eq!(v.zxyx(), i8vec4(3_i8, 1_i8, 2_i8, 1_i8));
    assert_eq!(v.zxyy(), i8vec4(3_i8, 1_i8, 2_i8, 2_i8));
    assert_eq!(v.zxyz(), i8vec4(3_i8, 1_i8, 2_i8, 3_i8));
    assert_eq!(v.zxzx(), i8vec4(3_i8, 1_i8, 3_i8, 1_i8));
    assert_eq!(v.zxzy(), i8vec4(3_i8, 1_i8, 3_i8, 2_i8));
    assert_eq!(v.zxzz(), i8vec4(3_i8, 1_i8, 3_i8, 3_i8));
    assert_eq!(v.zyxx(), i8vec4(3_i8, 2_i8, 1_i8, 1_i8));
    assert_eq!(v.zyxy(), i8vec4(3_i8, 2_i8, 1_i8, 2_i8));
    assert_eq!(v.zyxz(), i8vec4(3_i8, 2_i8, 1_i8, 3_i8));
    assert_eq!(v.zyyx(), i8vec4(3_i8, 2_i8, 2_i8, 1_i8));
    assert_eq!(v.zyyy(), i8vec4(3_i8, 2_i8, 2_i8, 2_i8));
    assert_eq!(v.zyyz(), i8vec4(3_i8, 2_i8, 2_i8, 3_i8));
    assert_eq!(v.zyzx(), i8vec4(3_i8, 2_i8, 3_i8, 1_i8));
    assert_eq!(v.zyzy(), i8vec4(3_i8, 2_i8, 3_i8, 2_i8));
    assert_eq!(v.zyzz(), i8vec4(3_i8, 2_i8, 3_i8, 3_i8));
    assert_eq!(v.zzxx(), i8vec4(3_i8, 3_i8, 1_i8, 1_i8));
    assert_eq!(v.zzxy(), i8vec4(3_i8, 3_i8, 1_i8, 2_i8));
    assert_eq!(v.zzxz(), i8vec4(3_i8, 3_i8, 1_i8, 3_i8));
    assert_eq!(v.zzyx(), i8vec4(3_i8, 3_i8, 2_i8, 1_i8));
    assert_eq!(v.zzyy(), i8vec4(3_i8, 3_i8, 2_i8, 2_i8));
    assert_eq!(v.zzyz(), i8vec4(3_i8, 3_i8, 2_i8, 3_i8));
    assert_eq!(v.zzzx(), i8vec4(3_i8, 3_i8, 3_i8, 1_i8));
    assert_eq!(v.zzzy(), i8vec4(3_i8, 3_i8, 3_i8, 2_i8));
    assert_eq!(v.zzzz(), i8vec4(3_i8, 3_i8, 3_i8, 3_i8));
    assert_eq!(v.xxx(), i8vec3(1_i8, 1_i8, 1_i8));
    assert_eq!(v.xxy(), i8vec3(1_i8, 1_i8, 2_i8));
    assert_eq!(v.xxz(), i8vec3(1_i8, 1_i8, 3_i8));
    assert_eq!(v.xyx(), i8vec3(1_i8, 2_i8, 1_i8));
    assert_eq!(v.xyy(), i8vec3(1_i8, 2_i8, 2_i8));
    assert_eq!(v.xzx(), i8vec3(1_i8, 3_i8, 1_i8));
    assert_eq!(v.xzy(), i8vec3(1_i8, 3_i8, 2_i8));
    assert_eq!(v.xzz(), i8vec3(1_i8, 3_i8, 3_i8));
    assert_eq!(v.yxx(), i8vec3(2_i8, 1_i8, 1_i8));
    assert_eq!(v.yxy(), i8vec3(2_i8, 1_i8, 2_i8));
    assert_eq!(v.yxz(), i8vec3(2_i8, 1_i8, 3_i8));
    assert_eq!(v.yyx(), i8vec3(2_i8, 2_i8, 1_i8));
    assert_eq!(v.yyy(), i8vec3(2_i8, 2_i8, 2_i8));
    assert_eq!(v.yyz(), i8vec3(2_i8, 2_i8, 3_i8));
    assert_eq!(v.yzx(), i8vec3(2_i8, 3_i8, 1_i8));
    assert_eq!(v.yzy(), i8vec3(2_i8, 3_i8, 2_i8));
    assert_eq!(v.yzz(), i8vec3(2_i8, 3_i8, 3_i8));
    assert_eq!(v.zxx(), i8vec3(3_i8, 1_i8, 1_i8));
    assert_eq!(v.zxy(), i8vec3(3_i8, 1_i8, 2_i8));
    assert_eq!(v.zxz(), i8vec3(3_i8, 1_i8, 3_i8));
    assert_eq!(v.zyx(), i8vec3(3_i8, 2_i8, 1_i8));
    assert_eq!(v.zyy(), i8vec3(3_i8, 2_i8, 2_i8));
    assert_eq!(v.zyz(), i8vec3(3_i8, 2_i8, 3_i8));
    assert_eq!(v.zzx(), i8vec3(3_i8, 3_i8, 1_i8));
    assert_eq!(v.zzy(), i8vec3(3_i8, 3_i8, 2_i8));
    assert_eq!(v.zzz(), i8vec3(3_i8, 3_i8, 3_i8));
    assert_eq!(v.xx(), i8vec2(1_i8, 1_i8));
    assert_eq!(v.xy(), i8vec2(1_i8, 2_i8));
    assert_eq!(v.xz(), i8vec2(1_i8, 3_i8));
    assert_eq!(v.yx(), i8vec2(2_i8, 1_i8));
    assert_eq!(v.yy(), i8vec2(2_i8, 2_i8));
    assert_eq!(v.yz(), i8vec2(2_i8, 3_i8));
    assert_eq!(v.zx(), i8vec2(3_i8, 1_i8));
    assert_eq!(v.zy(), i8vec2(3_i8, 2_i8));
    assert_eq!(v.zz(), i8vec2(3_i8, 3_i8));
});

glam_test!(test_i8vec2_swizzles, {
    let v = i8vec2(1_i8, 2_i8);
    assert_eq!(v, v.xy());
    assert_eq!(v.xxxx(), i8vec4(1_i8, 1_i8, 1_i8, 1_i8));
    assert_eq!(v.xxxy(), i8vec4(1_i8, 1_i8, 1_i8, 2_i8));
    assert_eq!(v.xxyx(), i8vec4(1_i8, 1_i8, 2_i8, 1_i8));
    assert_eq!(v.xxyy(), i8vec4(1_i8, 1_i8, 2_i8, 2_i8));
    assert_eq!(v.xyxx(), i8vec4(1_i8, 2_i8, 1_i8, 1_i8));
    assert_eq!(v.xyxy(), i8vec4(1_i8, 2_i8, 1_i8, 2_i8));
    assert_eq!(v.xyyx(), i8vec4(1_i8, 2_i8, 2_i8, 1_i8));
    assert_eq!(v.xyyy(), i8vec4(1_i8, 2_i8, 2_i8, 2_i8));
    assert_eq!(v.yxxx(), i8vec4(2_i8, 1_i8, 1_i8, 1_i8));
    assert_eq!(v.yxxy(), i8vec4(2_i8, 1_i8, 1_i8, 2_i8));
    assert_eq!(v.yxyx(), i8vec4(2_i8, 1_i8, 2_i8, 1_i8));
    assert_eq!(v.yxyy(), i8vec4(2_i8, 1_i8, 2_i8, 2_i8));
    assert_eq!(v.yyxx(), i8vec4(2_i8, 2_i8, 1_i8, 1_i8));
    assert_eq!(v.yyxy(), i8vec4(2_i8, 2_i8, 1_i8, 2_i8));
    assert_eq!(v.yyyx(), i8vec4(2_i8, 2_i8, 2_i8, 1_i8));
    assert_eq!(v.yyyy(), i8vec4(2_i8, 2_i8, 2_i8, 2_i8));
    assert_eq!(v.xxx(), i8vec3(1_i8, 1_i8, 1_i8));
    assert_eq!(v.xxy(), i8vec3(1_i8, 1_i8, 2_i8));
    assert_eq!(v.xyx(), i8vec3(1_i8, 2_i8, 1_i8));
    assert_eq!(v.xyy(), i8vec3(1_i8, 2_i8, 2_i8));
    assert_eq!(v.yxx(), i8vec3(2_i8, 1_i8, 1_i8));
    assert_eq!(v.yxy(), i8vec3(2_i8, 1_i8, 2_i8));
    assert_eq!(v.yyx(), i8vec3(2_i8, 2_i8, 1_i8));
    assert_eq!(v.yyy(), i8vec3(2_i8, 2_i8, 2_i8));
    assert_eq!(v.xx(), i8vec2(1_i8, 1_i8));
    assert_eq!(v.yx(), i8vec2(2_i8, 1_i8));
    assert_eq!(v.yy(), i8vec2(2_i8, 2_i8));
});
