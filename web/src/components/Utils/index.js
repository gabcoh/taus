import { useRef, forwardRef, useEffect, useState } from "react";
import { useTable, useRowSelect } from "react-table";
import { Table } from "react-bootstrap";

export function validateRegex(s) {
    try {
        new RegExp(s);
    } catch (err) {
        return `${err}`;
    }
    return undefined;
}

export function useRefresh() {
    const [ref, setRef] = useState(0);
    return [ref, () => setRef((r) => r + 1)];
}

// export function EditableTable({metadata, initialData}) {
//     const [data, setData] = useState(initialData);
//     const initDisDat = [];
//     for (let i = 0; i < initialData.length; i++) {
// 	let row = {};
// 	for (let k in metadata) {
// 	    row[k] = {
// 		"editing": false
// 	    };
// 	}
// 	initDisDat.push(row);
//     }
//     const [displayData, setDisplayData] = useState(initDisDat);

//     const toggleDisplay = (i, k) => {
// 	setDisplayData((data) => {
// 	    const newD = cloneDeep(data);
// 	    newD[i][k].editing = !newD[i][k].editing;
// 	    return newD;
// 	});
//     };
//     const displayCell = (i, k) => {
// 	if (displayData[i][k].editing && !metadata[k].readonly) {
// 	    return <input value={data[i][k]}
// 			  onClick={e => e.stopPropagation()}
// 			  onChange={e => setData((data) => {
// 			      const newD = cloneDeep(data);
// 			      newD[i][k] = e.target.value;
// 			      return newD;
// 			  })} />;
// 	} else {
// 	    return data[i][k];
// 	}
//     };
//     return (
// 	 <Container>
// 	     <Row>
// 		 <Col>
// 	 <Table bordered hover>
// 	     <thead>
// 		 <tr>
// 		     {Object.keys(metadata).map((k, i) => <th key={i}>{k}</th>)}
// 		 </tr>
// 	     </thead>
// 	     <tbody>
// 		 {data.map((t, it) => <tr key={it}>
// 					     {Object.keys(metadata).map((k, ik) =>
// 						 <td key={ik}
// 						     onClick={() => toggleDisplay(it, k)}
// 						 >{displayCell(it, k)}</td>
// 					     )}
// 					</tr>)}
// 	     </tbody>
// 	 </Table>
// 		 </Col>
// 		 </Row>
// 	     <Row>
// 		 <Col xs={{span:1, offset:11}}>
// 		     <Button variant="light" style={{"padding": "0"}}><h3> <BsFillPlusSquareFill className="mag-on-hov"/></h3></Button>
// 		 </Col>
// 	     </Row>
// 	 </Container>
//     );
// }

const EditableCell = ({
    value: initialValue,
    row: { index },
    column,
    updateData, // This is a custom function that we supplied to our table instance
}) => {
    // We need to keep and update the state of the cell normally
    const [value, setValue] = useState(initialValue);

    const onChange = (e) => {
        setValue(e.target.value);
    };

    // We'll only update the external data when the input is blurred
    const onBlur = () => {
        updateData(index, column.id, value);
    };

    // If the initialValue is changed external, sync it up with our state
    useEffect(() => {
        setValue(initialValue);
    }, [initialValue]);

    if (column.readOnly) {
        return value;
    } else {
        return <input value={value} onChange={onChange} onBlur={onBlur} />;
    }
};
const defaultColumn = {
    Cell: EditableCell,
};
export function EditableTable({ data, columns, updateData, skipPageReset }) {
    const tableInstance = useTable(
        {
            columns,
            data,
            autoResetPage: !skipPageReset,
            defaultColumn,
            updateData,
        },
        useRowSelect,
        (hooks) => {
            hooks.visibleColumns.push((columns) => [
                // Let's make a column for selection
                {
                    id: "selection",
                    // The header can use the table's getToggleAllRowsSelectedProps method
                    // to render a checkbox
                    Header: ({ getToggleAllRowsSelectedProps }) => (
                        <div>
                            <IndeterminateCheckbox
                                {...getToggleAllRowsSelectedProps()}
                            />
                        </div>
                    ),
                    // The cell can use the individual row's getToggleRowSelectedProps method
                    // to the render a checkbox
                    Cell: ({ row }) => (
                        <div>
                            <IndeterminateCheckbox
                                {...row.getToggleRowSelectedProps()}
                            />
                        </div>
                    ),
                },
                ...columns,
            ]);
        }
    );
    const {
        getTableProps,
        getTableBodyProps,
        headerGroups,
        rows,
        prepareRow,
        selectedFlatRows,
    } = tableInstance;
    return (
        <Table bordered hover {...getTableProps()}>
            <thead>
                {
                    // Loop over the header rows
                    headerGroups.map((headerGroup) => (
                        // Apply the header row props
                        <tr {...headerGroup.getHeaderGroupProps()}>
                            {
                                // Loop over the headers in each row
                                headerGroup.headers.map((column) => (
                                    // Apply the header cell props
                                    <th {...column.getHeaderProps()}>
                                        {
                                            // Render the header
                                            column.render("Header")
                                        }
                                    </th>
                                ))
                            }
                        </tr>
                    ))
                }
            </thead>
            {/* Apply the table body props */}
            <tbody {...getTableBodyProps()}>
                {
                    // Loop over the table rows
                    rows.map((row) => {
                        // Prepare the row for display
                        prepareRow(row);
                        return (
                            // Apply the row props
                            <tr {...row.getRowProps()}>
                                {
                                    // Loop over the rows cells
                                    row.cells.map((cell) => {
                                        // Apply the cell props
                                        return (
                                            <td {...cell.getCellProps()}>
                                                {
                                                    // Render the cell contents
                                                    cell.render("Cell")
                                                }
                                            </td>
                                        );
                                    })
                                }
                            </tr>
                        );
                    })
                }
            </tbody>
        </Table>
    );
}
export const IndeterminateCheckbox = forwardRef(
    ({ indeterminate, ...rest }, ref) => {
        const defaultRef = useRef();
        const resolvedRef = ref || defaultRef;

        useEffect(() => {
            resolvedRef.current.indeterminate = indeterminate;
        }, [resolvedRef, indeterminate]);

        return (
            <>
                <input type="checkbox" ref={resolvedRef} {...rest} />
            </>
        );
    }
);
