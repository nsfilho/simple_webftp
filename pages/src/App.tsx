import React from 'react'
import axios from 'axios';
import { toast } from 'react-toastify'
import DataTable from 'datatables.net-dt';
import { API_URL } from './constants'
import './App.css'
import 'datatables.net-dt/css/jquery.dataTables.min.css';

interface FileListResponseItem {
    name: string;
    size: number;
    createdAt: string;
    modifiedAt: string;
}

interface FormatedItem {
    name: string;
    size: string;
    createdAt: string;
    modifiedAt: string;
}

function App() {
    const list = React.useRef<HTMLTableElement>(null);
    const [file, setFile] = React.useState<File | null>(null);
    const [availablesFiles, setAvailablesFiles] = React.useState<FormatedItem[]>([]);

    React.useEffect(() => {
        if (list.current) {
            let table = new DataTable(list.current, {
                data: availablesFiles,
                columns: [
                    { title: 'Name', data: 'name', render: (data: string) => `<a href="${API_URL}/files/download/${data}">${data}</a>` },
                    { title: 'Size', data: 'size' },
                    { title: 'Created at', data: 'createdAt' },
                    { title: 'Modified at', data: 'modifiedAt' },
                ],
                destroy: true,
                paging: false,
                searching: true,
                info: false,
            });

            return () => {
                table.destroy();
            }
        }
    }, [list, availablesFiles]);

    React.useEffect(() => {
        let lastHash = '';
        const handleCheckInterval = setInterval(() => {
            axios.get<FileListResponseItem[]>(`${API_URL}/files/list`)
                .then(response => {
                    // sort by name 
                    response.data.sort((a, b) => {
                        if (a.name < b.name) return -1;
                        if (a.name > b.name) return 1;
                        return 0;
                    });
                    
                    // localize createdAt and modifiedAt, format size (bytes length) to a human readable format
                    let formated = response.data.map(item => {
                        const createdAt = new Date(item.createdAt).toLocaleString();
                        const modifiedAt = new Date(item.modifiedAt).toLocaleString();
                        const size = new Intl.NumberFormat('en-US').format(item.size);
                        return { ...item, createdAt, modifiedAt, size };
                    });

                    const currentHash = JSON.stringify(formated);
                    if (lastHash === currentHash) return;

                    // updating
                    lastHash = currentHash;
                    setAvailablesFiles(formated);
                })
                .catch(error => {
                    toast.error(`Error checking files: ${error.message}`);
                });
        }, 1000);
        return () => {
            clearInterval(handleCheckInterval);
        }
    }, []);

    const handleFileChange = (event: React.ChangeEvent<HTMLInputElement>) => {
        const files = event.target.files;
        if (files) {
            setFile(files[0]);
        }
    }

    const handleSubmit = (event: React.FormEvent<HTMLFormElement>) => {
        event.preventDefault();
        if (file) {
            const formData = new FormData();
            formData.append('file', file);
            axios.post(`${API_URL}/files/upload`, formData)
                .then(() => {
                    toast.success('File uploaded successfully');
                    setFile(null);
                })
                .catch(error => {
                    toast.error(`Error uploading file: ${error.message}`);
                });
        }
    }

    return (
        <>
            <form onSubmit={handleSubmit}>
                <h3>Simple WebFTP</h3>
                <div className="container">
                    <h3>File to upload</h3>
                    <div className="upload">
                        <input type="file" onChange={handleFileChange} />
                        {file && <button>Enviar</button>}
                    </div>
                    {file && (
                        <section>
                            <span className="fileDetails">File details:</span>
                            <ul>
                                <li>Name: {file.name}</li>
                                <li>Type: {file.type}</li>
                                <li>Size: {file.size} bytes</li>
                            </ul>
                        </section>
                    )}
                    <div className="files">
                        <h3>Availables files</h3>
                        <table ref={list} />
                    </div>
                </div>
            </form>
        </>
    )
}

export default App;
