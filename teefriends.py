#!/usr/bin/env python3
import requests, os, sys, argparse
from platformdirs import *

API_URL = "https://master1.ddnet.tw/ddnet/15/servers.json"

server_list = []
friend_name_list = []
active_friend_list = []

data_dir = user_data_dir()

ddnet_settings_path = data_dir + "/ddnet/settings_ddnet.cfg"
if not os.path.exists(ddnet_settings_path):
    print("[ERROR] Could not open ddnet config")

teefriends_store_path = data_dir + "/teefriends"
if not os.path.exists(teefriends_store_path):
    os.mkdir(teefriends_store_path)

def get_data():
    global server_list
    server_list = requests.get(API_URL).json().get("servers")

    settings = open(ddnet_settings_path)
    for line in settings:
        if "add_friend" in line:
            friend_name_list.append(line.split("\"")[1])

def fill_active_friend_list():
    for server in server_list:
        for client in server.get("info").get("clients"):
            for friend_name in friend_name_list:
                if client.get("name") == friend_name:
                    active_friend_list.append(client.get("name"))

def store_data():
    data = open(teefriends_store_path + "/friends.txt", "w")
    for name in active_friend_list:
        data.write(name + "\n")

def get_store_data():
    if not os.path.isfile(teefriends_store_path + "/friends.txt"):
        open(teefriends_store_path + "/friends.txt", "x")
    data = open(teefriends_store_path + "/friends.txt", "r")
    active_friend_list.clear()
    for name in data:
        active_friend_list.append(name)

def print_friend_count():
    print(len(active_friend_list))

def print_active_friend_list():
    for friend in active_friend_list:
        print(friend, end="")

parser = argparse.ArgumentParser(description='Get teeworlds server info and print it!')
parser.add_argument('-f', '--fetch', action='store_true', help='Send request to master server and get friend data')
parser.add_argument('-c', '--count', action='store_true', help='Print active friend count')
parser.add_argument('-n', '--names', action='store_true', help='Print active friend names')
parser.parse_args(args=None if sys.argv[1:] else ['--help'])

args = parser.parse_args()

if args.fetch:
    get_data()
    fill_active_friend_list()
    store_data()

if args.count:
    get_store_data()
    print_friend_count()

if args.names:
    get_store_data()
    print_active_friend_list()
