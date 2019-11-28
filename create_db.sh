#!/bin/bash
DB="your postgres uri here";
psql $DB -f "./create_tables.sql";