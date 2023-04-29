#!/bin/bash

echo -n "Sock: "
grep Sock lines | wc -l

echo -n "Sent: "
grep Sent lines | wc -l

echo -n "Got: "
grep Got lines | wc -l

echo -n "None: "
grep None lines | wc -l

