#include <iostream>
#include <filesystem>
#include <vector>
#include <algorithm>
#include <stdio.h>

int main(int argc, char *argv[]){
    bool force = false;
    std::string path_str;
    for (int i = 1; i < argc; i++) {
        std::string arg = argv[i];
        if (arg == "-f") {
            force = true;
        } else {
            path_str = arg;
        }
    }
    if (!force) {
        std::cout << path_str + ", Are you sure (y)? ";
        std::string input;
        std::cin >> input;
        if (input != "y") {
            return 0;
        }
    }
    std::cout << "Processing..." << "\n";
    std::vector<std::filesystem::path> list;
    const std::filesystem::path& path = path_str;
    for(const auto& p: std::filesystem::recursive_directory_iterator(path)) {
        list.push_back(p.path());
    }
    std::reverse(list.begin(), list.end());
    for (const auto& p: list) {
        std::string parent_path = p.parent_path().string();
        std::string filename = p.filename().string();

        std::string filename_lower;
        filename_lower.resize(filename.size());
        transform(filename.begin(), filename.end(), filename_lower.begin(), ::tolower);

        std::string oldpath = parent_path + "/" + filename;
        std::string newpath = parent_path + "/" + filename_lower;

        if (oldpath != newpath) {
            if (rename(oldpath.c_str(), newpath.c_str()) != 0) {
                printf("Could not rename %s to %s.\n", oldpath.c_str(), newpath.c_str());
                continue;
            }
            if (std::filesystem::exists(oldpath)) {
                // FAT16, FAT32, exFAT or NTFS
                srand((unsigned)time(NULL));
                std::string rnd = std::to_string(rand());
                std::string intermediatepath = parent_path + "/" + filename + "_rename_" + rnd;
                if (rename(oldpath.c_str(), intermediatepath.c_str()) != 0) {
                    printf("Could not rename %s to %s.\n", oldpath.c_str(), intermediatepath.c_str());
                    continue;
                }
                if (rename(intermediatepath.c_str(), newpath.c_str()) != 0) {
                    printf("Could not rename %s to %s.\n", intermediatepath.c_str(), newpath.c_str());
                    continue;
                }
            }
        }
    }
}
