# 关于gh-md-toc的使用

1. 量不大的时候不需要token.txt
2. 量大的时候需要添加github access token 到token.txt
3. 情况二的前提下，如果token.txt不小心加入到git上传到github，会被github自动检测后token失效，也不能使用。
4. 这时可以在github上面的分支设置环境变量：Settings -> Security -> Actions -> Add Repository secrets