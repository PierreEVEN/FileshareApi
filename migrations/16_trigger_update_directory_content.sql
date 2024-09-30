CREATE OR REPLACE PROCEDURE SCHEMA_NAME.update_all_directory_sizes() AS $$
	DECLARE
	BEGIN
		UPDATE SCHEMA_NAME.directories dir SET (num_items, content_size) = (SELECT COUNT(id), SUM(size) FROM SCHEMA_NAME.files
			WHERE id IN (SELECT id FROM SCHEMA_NAME.items WHERE is_regular_file = true AND STARTS_WITH(absolute_path, (SELECT absolute_path FROM SCHEMA_NAME.items WHERE id = dir.id))));
	END;
	$$ LANGUAGE plpgsql;

-- UPDATE ALL
CREATE OR REPLACE PROCEDURE SCHEMA_NAME.update_all_directory_sizes() AS $$
	DECLARE
		leaf_dirs RECORD;
		leaf_content RECORD;
	BEGIN
	    FOR leaf_dirs IN (SELECT * FROM SCHEMA_NAME.directories WHERE id NOT IN (SELECT parent_item FROM SCHEMA_NAME.items WHERE is_regular_file = false AND parent_item IS NOT NULL)) LOOP
            SELECT COUNT(id) AS num, SUM(size) AS size INTO leaf_content FROM SCHEMA_NAME.files WHERE id IN (SELECT id FROM SCHEMA_NAME.items WHERE parent_item = leaf_dirs.id);
            UPDATE SCHEMA_NAME.directories SET num_items = leaf_content.num, content_size = leaf_content.size WHERE id = leaf_dirs.id;
	    END LOOP;
	END;
	$$ LANGUAGE plpgsql;

-- UPDATE ITEM
CREATE OR REPLACE FUNCTION SCHEMA_NAME.update_directories_content_update() RETURNS TRIGGER AS $$
	DECLARE
		rec RECORD;
	BEGIN
        IF (OLD.parent_item IS NULL AND NEW.parent_item IS NOT NULL) OR
           (OLD.parent_item IS NOT NULL AND NEW.parent_item IS NULL) OR
           (OLD.parent_item != NEW.parent_item) THEN
           IF NEW.is_regular_file THEN
                SELECT size INTO rec FROM SCHEMA_NAME.files WHERE id = NEW.id;
                IF OLD.parent_item IS NOT NULL THEN
			        CALL SCHEMA_NAME.add_delta_on_directory(OLD.parent_item, -1, -rec.size);
                END IF;
                IF NEW.parent_item IS NOT NULL THEN
			        CALL SCHEMA_NAME.add_delta_on_directory(NEW.parent_item, 1, rec.size);
                END IF;
           ELSE
                SELECT num_items, content_size INTO rec FROM SCHEMA_NAME.directories WHERE id = NEW.id;
                IF OLD.parent_item IS NOT NULL THEN
			        CALL SCHEMA_NAME.add_delta_on_directory(OLD.parent_item, -rec.num_items, -rec.content_size);
                END IF;
                IF NEW.parent_item IS NOT NULL THEN
			        CALL SCHEMA_NAME.add_delta_on_directory(NEW.parent_item, rec.num_items, rec.content_size);
                END IF;
           END IF;
        END IF;
		RETURN NEW;
	END;
	$$ LANGUAGE plpgsql;

CREATE OR REPLACE TRIGGER trig_update_directories_content_update
	AFTER UPDATE ON SCHEMA_NAME.items
	FOR EACH ROW EXECUTE FUNCTION SCHEMA_NAME.update_directories_content_update();

-- REMOVE FILE
CREATE OR REPLACE FUNCTION SCHEMA_NAME.trig_update_directories_content_remove() RETURNS TRIGGER AS $$
	DECLARE
		parent_id BIGINT;
	BEGIN
        SELECT parent_item INTO parent_id FROM SCHEMA_NAME.items WHERE id = OLD.id;
        IF parent_id IS NOT NULL THEN
			CALL SCHEMA_NAME.add_delta_on_directory(parent_id, -1, -OLD.size);
        END IF;
		RETURN OLD;
	END;
	$$ LANGUAGE plpgsql;

CREATE OR REPLACE TRIGGER trig_update_directories_content_remove
	BEFORE DELETE ON SCHEMA_NAME.files
	FOR EACH ROW EXECUTE FUNCTION SCHEMA_NAME.trig_update_directories_content_remove();

-- ADD FILE
CREATE OR REPLACE FUNCTION SCHEMA_NAME.trig_update_directories_content_insert() RETURNS TRIGGER AS $$
	DECLARE
		parent_id BIGINT;
	BEGIN
        SELECT parent_item INTO parent_id FROM SCHEMA_NAME.items WHERE id = NEW.id;
        IF parent_id IS NOT NULL THEN
			CALL SCHEMA_NAME.add_delta_on_directory(parent_id, 1, NEW.size);
        END IF;
		RETURN NEW;
	END;
	$$ LANGUAGE plpgsql;

CREATE OR REPLACE TRIGGER trig_update_directories_content_insert
	AFTER INSERT ON SCHEMA_NAME.files
	FOR EACH ROW EXECUTE FUNCTION SCHEMA_NAME.trig_update_directories_content_insert();