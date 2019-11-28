#!/bin/bash
DB="your postgres uri here";
psql $DB -f "./delete_tables.sql";