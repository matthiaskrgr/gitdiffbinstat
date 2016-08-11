#!/usr/bin/env python3

#    gitdiffbinstat - gets a git diff --shortstat-like output for changed binary files
#    Copyright (C) 2016  Matthias Krüger

#    This program is free software; you can redistribute it and/or modify
#    it under the terms of the GNU General Public License as published by
#    the Free Software Foundation; either version 1, or (at your option)
#    any later version.

#    This program is distributed in the hope that it will be useful,
#    but WITHOUT ANY WARRANTY; without even the implied warranty of
#    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
#    GNU General Public License for more details.

#    You should have received a copy of the GNU General Public License
#    along with this program; if not, write to the Free Software
#    Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston MA  02110-1301 USA

__author__ = 'Matthias "matthiaskrgr" Krüger'


import sys
if (sys.version_info.major != 3): # no python 3
	print("Python 3 or higher is required.")
	sys.exit(1)

import locale # locale.setlocale
import argparse #argument parsing
import subprocess # calling git stuff
from subprocess import *

def color(string, color):
	# returns colored string
	END = "\[0m" # resets color
	if (color == "red"):
		string = "\033[31m" + string
	elif (color == "green"):
		string = "\[033;32m" + string
	elif (color == "greenul"):
		string = "\[4;32m" + string
	elif (color == "whiteul"):
		string = "\[4;02m" + string
# reset color
	return string + END

assert(color("hi", "red") == "\033[31mhi\[0m")
assert(color("foo", "whiteul") + color("baz", "greenul") == "\[4;02mfoo\[0m\[4;32mbaz\[0m")

locale.setlocale(locale.LC_ALL, 'en_GB') # set locale (export LANG=C)


def print_fatal(text):
	# prints to stderr and exits program
	sys.stderr.write(text + "\n")
	sys.exit(1)

# @TODO: make sure we find git executable


def check_cmd_exitstatus_is_false(cmd):
	# @param cmd string
	proc = Popen(cmd, stderr=STDOUT, stdout=DEVNULL, shell=True)
	proc.communicate()
	return True if (proc.returncode != 0) else False # ternary op


def assert_inside_git_repo():
	if (check_cmd_exitstatus_is_false("git rev-parse --is-inside-work-tree")):
		print_fatal("ERROR: not inside git repository")


def assert_arg_is_known_by_git(gitobj):
	if (check_cmd_exitstatus_is_false("git rev-parse --quiet --verify " + gitobj)):
		print_fatal("ERROR: '" + gitobj + "' is not known to git.")

def get_git_hash_from_obj(gitobj):
	assert_arg_is_known_by_git(gitobj)
	hash_ = subprocess.Popen("git log -1 --format=%H " + gitobj, stdout=subprocess.PIPE, shell=True).stdout.read().decode('utf8')
	return hash_.rstrip('\n') # remove newline

assert_inside_git_repo()


parser = argparse.ArgumentParser()
parser.add_argument("gitobj", help="a branch, or a arange of commits (commit1..commit2)", metavar='<commit/tag/branch/commit range>', nargs=1, type=str)
args = parser.parse_args()
GITOBJ = args.gitobj[0]

#assert_arg_is_known_by_git(GITOBJ)


# split up the arg, if needed
arg1=None
arg2=None
if ("..." in GITOBJ):
	arg1 = GITOBJ.split("...")[0]
	arg2 = GITOBJ.split("...")[1]
elif (".." in GITOBJ):
	arg1 = GITOBJ.split("..")[0]
	arg2 = GITOBJ.split("..")[1]
else:  # assume we diff HEAD against something
	arg1 = GITOBJ

if (not arg2): # we need to get the HEAD
	arg2="HEAD"

assert_arg_is_known_by_git(arg1)
assert_arg_is_known_by_git(arg2)

CURBRANCH = arg1
OBJ = arg2
CURCOMMIT=get_git_hash_from_obj(arg1)
OBJHASH=get_git_hash_from_obj(arg2)

print(CURBRANCH + ".." + OBJ)
print(CURCOMMIT + ".." + OBJHASH)
