# Load the dictionary back from the pickle file.
import pickle
import pprint
ast = pickle.load(open("t.pickle", "rb"))
pp = pprint.PrettyPrinter(indent=4)
pp.pprint(ast)
