import pandas as pd
import numpy as np
from sklearn.cluster import KMeans

# 读取数据文件
df = pd.read_csv('oneday/code/out_publisher_embeding')

# 将字符串格式的嵌入转换为数值格式
df['sentence_embeddings'] = df['sentence_embeddings'].apply(eval)

# 提取嵌入数据并转换为NumPy数组
embeddings = np.array(df['sentence_embeddings'].tolist())

# 使用KMeans算法进行聚类
num_clusters = 2500  # 你可以根据需求调整聚类的数量
kmeans = KMeans(n_clusters=num_clusters, random_state=42)
df['cluster'] = kmeans.fit_predict(embeddings)

# 将聚类结果保存到新的CSV文件
df[["sentence","cluster"]].to_csv('oneday/code/data_with_clusters.csv', index=False)

print("聚类完成，结果已保存到 data_with_clusters.csv")
