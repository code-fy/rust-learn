from text2vec import SentenceModel
m = SentenceModel()
emb = m.encode("com")
print(emb)