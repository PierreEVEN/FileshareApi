
CREATE OR REPLACE FUNCTION fileshare_v3.remove_item(item BIGINT) RETURNS BIGINT[] AS $$
	DECLARE
		removed_objects BIGINT[] := '{}';
		entry RECORD;
		removed_file BIGINT;
		removed_object BIGINT;
	BEGIN
	    FOR entry IN SELECT id FROM fileshare_v3.items WHERE parent_item = item LOOP
			removed_objects := removed_objects || fileshare_v3.remove_item(entry.id);
		END LOOP;

		SELECT object INTO removed_file FROM fileshare_v3.file WHERE id = item;
		DELETE FROM fileshare_v3.items WHERE id = item;

		IF removed_file IS NOT NULL THEN
			SELECT id INTO removed_object FROM fileshare_v3.objects WHERE id NOT IN (SELECT id FROM fileshare_v3.objects WHERE id = removed_file);
			removed_objects := ARRAY_APPEND(removed_objects, removed_object);
		END IF;
	    RETURN removed_objects;
	END;
$$ LANGUAGE plpgsql;