import { useState, useEffect, useMemo } from "react";
import * as BS from "react-bootstrap";
import { BsFillPlusSquareFill } from "react-icons/bs";
import * as Formik from "formik";

import { validateRegex, EditableTable, useRefresh } from "../Utils";
import { createTarget, getTargets } from "../../crud";

const { Modal, Button } = BS;

export default function Targets() {
    const [error, setError] = useState();
    const [uploadError, setUploadError] = useState();
    const [targets, setTargets] = useState();
    const [skipPageReset, setSkipPageReset] = useState(false);
    const [ref, refresh] = useRefresh();
    const [creatingNew, setCreatingNew] = useState(false);
    useEffect(() => {
        getTargets()
            .then((data) => setTargets(data))
            .catch((err) => setError(err));
    }, [ref]);
    useEffect(() => {
        let int = setInterval(() => refresh(), 2000);
        return () => clearInterval(int);
    }, []);

    const handleClose = () => setCreatingNew(false);

    const columns = useMemo(
        () => [
            {
                Header: "Target",
                accessor: "target",
            },
            {
                Header: "Regex",
                accessor: "regex",
            },
	    {
		Header: "Latest",
		accessor: "latest"
	    }
        ],
        []
    );

    const validate = (values) => {
        const errors = {};
        let err = validateRegex(values.regex);
        if (err) {
            errors.regex = err;
        }
        return errors;
    };

    return (
        <>
            <h3>Targets</h3>
            {error && (
                <BS.Alert variant="danger">
                    Error connecting to backend: {`${error}`}
                </BS.Alert>
            )}
            {targets && (
                <>
                    <EditableTable
                        data={targets}
                        columns={columns}
                        updateData={(ind, id, val) => {}}
                        skipPageReset={skipPageReset}
                    />
                    <Button
                        variant="light"
                        onClick={() => setCreatingNew(true)}
                    >
                        {" "}
                        <h3>
                            {" "}
                            <BsFillPlusSquareFill />{" "}
                        </h3>{" "}
                    </Button>

                    <Modal show={creatingNew} onHide={handleClose}>
                        <Formik.Formik
                            initialValues={{
                                target: "",
                                regex: "",
                            }}
                            validate={validate}
                            onSubmit={(values, { setSubmitting }) => {
                                console.log("Submitting");
                                setSubmitting(true);
                                createTarget(values)
                                    .then((resp) => {
                                        refresh();
                                        setUploadError();
                                        setCreatingNew(false);
                                    })
                                    .catch((error) => {
                                        setUploadError(error);
                                    })
                                    .finally(() => setSubmitting(false));
                            }}
                        >
                            {({ errors, submitForm }) => (
                                <>
                                    <Modal.Header closeButton>
                                        <Modal.Title>
                                            Create a new target
                                        </Modal.Title>
                                    </Modal.Header>
                                    <Modal.Body>
                                        <Formik.Form as={BS.Form}>
                                            <BS.Form.Group className="mb-3">
                                                <BS.Form.Label>
                                                    Target
                                                </BS.Form.Label>
                                                <Formik.Field
                                                    name="target"
                                                    as={BS.Form.Control}
                                                    placeholder="Enter target name"
                                                />
                                                <BS.Form.Text className="text-muted">
                                                    Something like 'win' or
                                                    'osx' or 'linux-amd64'
                                                </BS.Form.Text>
                                            </BS.Form.Group>
                                            <BS.Form.Group
                                                className="mb-3"
                                                controlId="formRegex"
                                            >
                                                <BS.Form.Label>
                                                    Regex
                                                </BS.Form.Label>
                                                <Formik.Field
                                                    isInvalid={errors.regex}
                                                    name="regex"
                                                    as={BS.Form.Control}
                                                    placeholder="app-name-("
                                                />
                                                <BS.Form.Control.Feedback type="invalid">
						    {`${errors.regex}`}
						</BS.Form.Control.Feedback>
                                            </BS.Form.Group>
                                            <BS.Form.Group>
                                                <BS.Form.Control
                                                    style={{
                                                        maxWidth: "100px",
                                                    }}
                                                    as={BS.Button}
                                                    variant="primary"
                                                    type="submit"
                                                    isInvalid={!!uploadError}
                                                >
                                                    Create Target
                                                </BS.Form.Control>
                                                <BS.Form.Control.Feedback type="invalid">
                                                    Creation failed:{" "}
                                                    {`${uploadError}`}
                                                </BS.Form.Control.Feedback>
                                            </BS.Form.Group>
                                        </Formik.Form>
                                    </Modal.Body>
                                </>
                            )}
                        </Formik.Formik>
                    </Modal>
                </>
            )}
        </>
    );
}
