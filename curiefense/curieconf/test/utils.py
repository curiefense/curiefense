from curieconf import utils

def compare_jblob(jblob1, jblob2):
    if jblob1 == jblob2:
        return True
    blob1 = utils.jblob2bytes(jblob1)
    blob2 = utils.jblob2bytes(jblob2)
    if blob1 == blob2:
        return True
    jjblob1 = utils.bytes2jblob(blob1)
    jjblob2 = utils.bytes2jblob(blob2)
    return jjblob1 == jjblob2

