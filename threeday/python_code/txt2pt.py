from typing import Dict,Any
import numpy as np
import torch
import torch.nn as nn
import json
from collections import defaultdict
from datetime import datetime
import re

def gen_dict(d):
    result = {}
    for key, value in d.items():
        prefix = key.split('\002')[0]
        suffix = key  
    
        if prefix not in result:
            result[prefix] = {}
    
            result[prefix][suffix] = value
    return result

device = torch.device('cuda') if torch.cuda.is_available() else torch.device(
    'cpu')

def parse_logistic_regression_model(file_path,dt_str):
    weights = {}
    with open(file_path, 'r') as file:
        next(file) 
        for line in file:
            parts = line.strip().split('\001')
            feature, weight = parts[0], float(parts[-1])
            weights[feature] = weight

    wide_weights = gen_dict(weights)

    prefix_set = {key.split('\002')[0] for key in weights.keys()}
    group_index = {prefix: i + 1 for i, prefix in enumerate(prefix_set)}
    group_index['0'] = 0

    fea_index = {key: i + 1 for i, key in enumerate(weights.keys())}
    fea_index['0\0020'] = 0

    result = defaultdict(lambda: {'idxes': {}, 'field_idx': {}})
    result_wide = defaultdict(lambda: {'idxes': {}})

    for key, value in fea_index.items():
        prefix = key.split('\002')[0]
        
        if prefix not in result:
            result[prefix] = {'idxes': {}, 'field_idx': group_index[prefix]}
        
        result[prefix]['idxes'][key] = len(result[prefix]['idxes']) + 1

    for key, value in fea_index.items():
        prefix = key.split('\002')[0]
        result_wide[prefix]['idxes'][key] = value

    fea_conf = {}
    fea_conf = {
        'embeding_dim':8,
 
         'wide': {
             
         },
        'deep': {
             
        }
     }

    fea_conf['wide']['fields'] = result_wide
    fea_conf['deep']['fields'] = result


    with open(file_path, 'r') as file:
        first_line = file.readline().strip()
    fields = [
    'featureCount', 'modelVersion', 'gloveModelVersion', 'time', 
    'seps', 'pushVersion', 'goalType', 'discretizationVersion'
]


    tmp_header = {}
    for field in fields:
        match = re.search(rf'{field}:(\S*)', first_line)
        if match:
            fea_conf[field] = match.group(1)
            tmp_header[field] = match.group(1)
        else:
            fea_conf[field] = None
            tmp_header[field] = None  

    # with open("fea_conf.json", "w") as json_file:
    #     json.dump(fea_conf, json_file, indent=4)
    # 序列特征进行占位，“A” 具体特征 3 截取个数
    tmp_header["sequence_features"] = [{"A":3},{"B":4}]
    tmp_header["both"] = "wide_only"

    
    # metadata = f"model_name=test\001model_cate=0\001model_version=0\001test_code=0\001group_num={len(group_index)}\001fid_num={len(weights)}\n"
    metadata = json.dumps(tmp_header)
    with open('feature_index.txt', 'w') as f:
        """
        txt file explanation: {k}, group feature name
        \001
        {v['field_idx']}, group feature id
        \001
        {kk}, feature value
        \001
        {vv}, feature id within the feature group
        \001
        {fea_conf['wide']['fields'][k]['idxes'][kk], feature id
        }\n
        search deep feature embedding: group feature id indicates the embedding module index
                                       feature id within the feature group indicates the specific embedding
                                       index within the embedding module to locate the current feature's specific embedding representation

        search wide feature embedding: feature id indicates the specific index of the wide side embedding
        X_w : {fea_conf['wide']['fields'][k]['idxes'][kk], feature id
        X_d_dict:Dict[str, torch.Tensor] : Dict[group feature id, feature id within the feature group]
        """
        f.write(metadata+'\n')
        for k,v in fea_conf['deep']['fields'].items():
            for kk,vv in v['idxes'].items():
                # line = f"{v['field_idx']}\001{kk}\001{vv}\001{fea_conf['wide']['fields'][k]['idxes'][kk]}\n"
                line = f"{-1}\001{kk}\001{-1}\001{fea_conf['wide']['fields'][k]['idxes'][kk]}\n"
                f.write(line)
        # if w:
        #     pass
        # if d:
        #     pass

    return fea_conf, wide_weights

class CustomModel(nn.Module):

    def __init__(self,fea_conf,wide_weights):

        super(CustomModel, self).__init__()
        self.device = device
        self.fea_conf = fea_conf
        embs = []
        self.emb_dim = self.fea_conf['embeding_dim']
        for field_name, field in self.fea_conf['deep']['fields'].items():
            weight = self._init_deep_embedding(field_name,
                                                     field['idxes'],
                                                     self.emb_dim,
                                                     {})

            emb = nn.Embedding(len(field['idxes']) + 1,
                               self.emb_dim,
                               padding_idx=0,
                               _weight=weight)
            emb._fill_padding_idx_with_zero()
            embs.append([str(field['field_idx']), emb])

        embs.sort(key=lambda e: int(e[0]))
        self.embeddings = nn.ModuleDict(embs)

        self.deep_field = len(self.fea_conf['deep']['fields'])
        self.deep_dim = self.emb_dim * self.deep_field
        self.output_size = self.deep_dim
        self.wide_dim = np.sum([
            len(field['idxes'])
            for field in self.fea_conf['wide']['fields'].values()
        ]).item()

        weight = self._init_wide_embedding(
            self.fea_conf['wide']['fields'], self.wide_dim, {'wide_params':wide_weights})
        self.wide_embedding = nn.Embedding(self.wide_dim + 1,
                                           1,
                                           padding_idx=0,
                                           _weight=weight)
        self.wide_embedding._fill_padding_idx_with_zero()


    def forward(self, X_w,X_d_dict:Dict[str, torch.Tensor]):
        emb = [
            torch.sum(field_emb(X_d_dict[i].long()), -2)
            for i, field_emb in self.embeddings.items()
        ]
        x_deep = torch.cat(emb, 1)

        # x_wide = self.wide_embedding(X_w)
        # x_wide = torch.sum(x_wide, 1)
        # x_wide = torch.squeeze(x_wide, dim=-1)

        return torch.sigmoid(torch.sum(self.wide_embedding(X_w))).item()
 


    def _init_deep_embedding(self, field_name, idxes, emb_size, emb_stat_dict):
        field_stat_dict = emb_stat_dict.get('deep_params',
                                            {}).setdefault(field_name, {})
        emb = torch.zeros((len(idxes) + 1, emb_size), device=self.device)
        torch.nn.init.normal_(emb, mean=0, std=0.001)
        for f_name, idx in idxes.items():
            if f_name in field_stat_dict:
                emb[idx[0]] = torch.tensor(field_stat_dict[f_name]['emb'])
        return emb
    
    def _init_wide_embedding(self, fields: Dict[str, Dict[str, any]],
                            fea_cnt: int, emb_stat_dict: Dict[str, any]):
        wide_stat_dict = emb_stat_dict.get('wide_params', {})
        emb = torch.zeros((int(fea_cnt + 1), 1), device=self.device)
        torch.nn.init.normal_(emb, mean=0, std=0.001)
        for field_name, field in fields.items():
            if field_name not in wide_stat_dict:
                continue
            field_stat_dict = wide_stat_dict[field_name]
            for fea_name, idx in field['idxes'].items():
                if fea_name in field_stat_dict:
                    emb[idx] = torch.tensor(
                        field_stat_dict[fea_name],dtype=torch.float64)

        return emb

def trans_save_pt(model):

    scripted_model = torch.jit.script(model)
    torch.jit.save(scripted_model, 'logistic_model_scripted.pt')
    print("Model has been converted to TorchScript format and saved as 'logistic_model_scripted.pt'.")

if __name__ == '__main__':
    # Any online txt model
    txt_model = '/Users/apple/Downloads/CL_IPR_ALLINONE.txt'
    dt_str = datetime.now().strftime("%Y%m%d")
    fea_config,wide_weights = parse_logistic_regression_model(txt_model,dt_str)
    model = CustomModel(fea_conf=fea_config,wide_weights=wide_weights)
    trans_save_pt(model=model)

  
    # test
    # x_w = [1, 2, 3, 4, 5, 6, 7, 8, 9]
    # X_d_dict = {}
    # for i in range(1000):
    #     X_d_dict[str(i)] = torch.tensor([[1.0]])
    # x_w_tensor = torch.tensor(x_w)
    # print(model.forward(x_w_tensor,X_d_dict))




