import zipfile

def create_oiv(rpf_path, version, output_file):
    with zipfile.ZipFile(f"{output_file}-{version[0]}.{version[1]}.oiv", 'w') as zipf:
        zipf.write(rpf_path, "content/american_rel.rpf")
        zipf.writestr('content/', '')
        zipf.write("assembly.xml")

version = (1,3)

create_oiv("american_rel.rpf", version, "GTA-V-Bahasa-Indonesia")