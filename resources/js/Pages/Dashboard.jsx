import AuthenticatedLayout from "@/Layouts/AuthenticatedLayout";
import InputError from "@/Components/InputError";
import PrimaryButton from "@/Components/PrimaryButton";

import { Head } from "@inertiajs/react";
import { useState } from "react";
import axios from "axios";

export default function Index({ auth }) {
    const [data, setData] = useState(null);
    const [error, setError] = useState("");

    const submit = async (e) => {
        e.preventDefault();

        const form = new FormData();
        form.append("csv", data);

        axios
            .post("http://localhost:8080/upload", form)
            .then((res) => {})
            .then((error) => setError(error));
    };

    return (
        <AuthenticatedLayout user={auth.user}>
            <Head title="Dashboard" />

            <main className="pt-10">
                <section className="w-[90%] md:w-[40%] mx-auto space-y-1">
                    <form
                        className="flex flex-col mx-auto items-center"
                        onSubmit={submit}
                    >
                        <input
                            required
                            type="file"
                            onChange={(e) => setData(e.target.files[0])}
                            className="file-input file-input-bordered w-full max-w-xs"
                        />
                        <InputError message={error} className="mt-2" />
                        <PrimaryButton
                            className="mt-4 mx-auto max-w-fit space-x-3"
                            // disabled={processing}
                        >
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                width="24"
                                height="24"
                                viewBox="0 0 24 24"
                                fill="none"
                                stroke="currentColor"
                                strokeWidth="2"
                                strokeLinecap="round"
                                strokeLinejoin="round"
                                className="lucide lucide-cloud-upload"
                            >
                                <path d="M4 14.899A7 7 0 1 1 15.71 8h1.79a4.5 4.5 0 0 1 2.5 8.242" />
                                <path d="M12 12v9" />
                                <path d="m16 16-4-4-4 4" />
                            </svg>
                            <span>Store Csv Data</span>
                        </PrimaryButton>
                    </form>
                </section>
            </main>
        </AuthenticatedLayout>
    );
}
