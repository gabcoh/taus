import { useEffect, useState, useMemo } from "react";

import { useRefresh, EditableTable, } from "../Utils";
import { getMappings } from '../../crud';

export default function Mappings() {
    const [skipPageReset, setSkipPageReset] = useState(false);
    const [mappings, setMappings] = useState([]);
    const [error, setError] = useState();
    const [ref, refresh] = useRefresh();
    useEffect(() => {
	getMappings()
	    .then(setMappings)
	    .catch(setError);
    }, [ref]);
    useEffect(() => {
	let int = setInterval(() => refresh(), 2000);
	return () => clearInterval(int);
    }, []);
    const columns = useMemo(
        () => [
            {
                Header: "Target ID",
                accessor: "target_id",
		readOnly: true,
            },
            {
                Header: "Version",
                accessor: "current_version",
		readOnly: true,
            },
            {
                Header: "Update Version",
                accessor: "update_version",
            },
        ],
        []
    );
    return (
        <>
            <h3>
                Mappings
            </h3>
            <EditableTable
                data={mappings}
                columns={columns}
                updateData={(ind, id, val) => {}}
                skipPageReset={skipPageReset}
            />
	    </>
    );
}
