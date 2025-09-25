## _TOZED_PASSWORDS
Project to understand the password generation of the TOZED routers

## Discriminatory Notice:
Please be aware that the information presented here is for educational purposes only. This project was undertaken as a personal hobby by an enthusiast who enjoys exploring technology. It is not intended for any malicious or illegal activities.

### Password Generation Reverse Engineering
Most TOZED brand routers were found to have three default user accounts with varying web UI access levels: `TZ_USER`,`TZ_SUPER`,`TZ_TEST`. It was observed that session tokens for these users were also created with the login username. It had also been noted that operators had a password when an IMEI number was provided to them.

Out of curiosity, the extracted firmware's from tozed S12 Pro binary was checked, and every configuration file was examined. It was discovered that the factory reset configuration comes from the tozed_param file in the /etc overlay directory. However, no direct, readable, hard-coded passwords were found within this file. A text string was found indicating that the password was `_USER_PWD_RANDOM="y"` along with a reference to a `_USER_PWD_RANDOM_WAY="1"`

The investigation then shifted to the device's bash scripts. A file was discovered that executed a compiled binary, `tozed_tool`, which saved a string into the tozed cfg configuration section. With the help of Ghidra, it was found that two random complex password generation methods and one random numerical only method were available, based on tozed_param options. The system was even found to support hard-coded passwords within the binary or in the tozed_param file, which were then saved as an MD5 hash in the tozed cfg configuration section.

After extensive googleing and with the help of AI, a working code was finally obtained for the generation of both the `Operator` and `sztozed (test user)` passwords.

It is **important** to note that the device tested had *firmware (version 1.23)* which was observed to block test user access if the device was in user mode, based on the file present in device. Given that most Sri Lankan operators' firmware is not updated and assuming every TOZED device has the same `tozed_param - config ver. S12U-SC001-v3.26UP`  file, there is a chance that the generated passwords could work on other devices.

Also the tozed_param file had checksum file that had 16 bit hash. 

# Again, This case study was done solely out of curiosity and for the joy of discovery. No commercial interest or illegal actions are being promoted.
