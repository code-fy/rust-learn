import torch

# 加载 TorchScript 模型
model = torch.jit.load('logistic_model_scripted.pt')

# 打印模型结构
# print(model)
x_w = [1, 2, 3, 4, 5, 6, 7, 8, 9]
X_d_dict = {'0': torch.tensor([[1.0], [2.0], [3.0]])}

# 将 x_w 转换为 PyTorch Tensor
x_w_tensor = torch.tensor(x_w)

# 调用 forward 函数
print(model(x_w_tensor, X_d_dict))

with torch.no_grad():  # 关闭梯度计算，以减少计算开销
    output = model(x_w_tensor, X_d_dict)

print("Model output:", output)

# # 打印模型的参数
# for name, param in model.named_parameters():
#     print(f"Parameter name: {name}")
#     print(f"Parameter value: {param.data}")
#     print(f"Parameter data length: {len(param.data.view(-1))}")






