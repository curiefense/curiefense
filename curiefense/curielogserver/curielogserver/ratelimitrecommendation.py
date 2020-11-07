import yaml

class FeatureAnalysis(object):

    def __init__(self, **kwargs):
        self.input_params = {}
        self.yaml_data = None
        self.input_params.update(kwargs)
        self.yaml_data = self._load_yaml()

    def _load_yaml(self):
        '''
        Read  yaml template from path
        @param file_name name of template
        return  yaml template
        '''
        full_path = self.input_params['yaml_file_name']
        try:
            with open(full_path, 'r') as reader:
                yaml_content = reader.read()
                return yaml.load(yaml_content,Loader=yaml.FullLoader)
        except Exception as error:
            print('failed loading yaml file {0}'.format(error))

    def _validate_input_params(self):
        for param in self.yaml_data['input_params']:
            name = param['name']
            _type = param['type']
            # a) validate param provided
            if name not in self.input_params:
                print('input param name {name} is missing'.format(name=name))
                return False
            # b) validate param data type
            input_type = type(self.input_params[name]).__name__
            if input_type != _type:
                print('input param name {name} type mismatch got {_type} while expecting {yaml_type}'.format(
                    name=name, _type=input_type, yaml_type=_type))
                return False
        return True

    def construct_sql(self):
        '''
        This function completed sql template
        '''
        valid_input = self._validate_input_params()
        if valid_input:
            sql_template = self.yaml_data['sql_template']
            if sql_template:
                try:
                    return sql_template.format(**self.input_params)
                except:
                    print('failed formatting sql_template from yaml data')
                    return None
            return None
        return None
        print('failed loading sql_template from yaml data')

    def _run_feature(self):
        sql = self.construct_sql()
        return sql

    def run(self):
        return self._run_feature()



class RateLimitLocation(FeatureAnalysis):

	def __init__(self, **kwargs):
		FeatureAnalysis.__init__(self, **kwargs)
		key_composition = self.input_params["key_composition"]
		include = self.input_params["include"]
		exclude = self.input_params["exclude"]
		self.input_params["gen_key_composition"] = self._gen_key_composition(key_composition)
		self.input_params["gen_include"] = self._gen_include(include)
		self.input_params["gen_exclude"] = self._gen_exclude(exclude)
	
	def _gen_key_composition (self,item):
		lines = []
		def comma2arrow(item):
				return "->".join(item[0:-1]) + "->>" + item[-1]
		keys = list(map(comma2arrow, item))
		def construct_key_composition():
			for key in keys:
			    lines.append("(curiefense->{key})".format(key=key))
			return "concat(" +  " , ".join(lines) + ")"
		key_composition_sql = construct_key_composition()
		return key_composition_sql

	def _gen_include (self,include_param):
		lines = []
		def comma2arrow(include_param):
				return "->".join(include_param[0:-1]) + "->>" + include_param[-1]
		keys = list(map(comma2arrow, include_param))
		def construct_include():
			for key in keys:
			    lines.append(" AND (curiefense->{key})".format(key=key))
			return " ".join(lines)
		return construct_include()

	def _gen_exclude (self,exclude_param):
		lines = []
		def comma2arrow(exclude_param):
				return "->".join(exclude_param[0:-1]) + "->>" + exclude_param[-1]
		keys = list(map(comma2arrow, exclude_param))
		def construct_exclude():
			for key in keys:
			    lines.append(" AND NOT (curiefense->{key})".format(key=key))
			return " ".join(lines)
		return construct_exclude()

	pass

def rate_limit_recommend(input_args):
	rate_limloc = RateLimitLocation(**input_args)
	result = rate_limloc.run()
	return result




  






      