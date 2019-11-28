DELETE FROM time_slices where competitor in (select id from competitors where image_id = $1);
DELETE FROM vulnerabilities where image_id = $1;
DELETE FROM competitors WHERE image_id = $1 ;
DELETE FROM images WHERE id = $1;