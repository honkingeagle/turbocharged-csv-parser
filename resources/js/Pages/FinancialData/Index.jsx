import AuthenticatedLayout from "@/Layouts/AuthenticatedLayout";

import { Head } from "@inertiajs/react";

export default function Index({ auth, financial_years }) {
    return (
        <AuthenticatedLayout user={auth.user}>
            <Head title="Financial Data" />

            <main className="pt-10">
                <section className="w-[90%] md:w-[60%] mx-auto space-y-1">
                    <div className="overflow-x-auto">
                        <table className="table table-md table-pin-rows table-pin-cols">
                            <thead>
                                <tr>
                                    <th></th>
                                    <th>Year</th>
                                    <th>Value</th>
                                    <th>Units</th>
                                    <th>Variable Name</th>
                                    <th>Variable Category</th>
                                </tr>
                            </thead>
                            <tbody>
                                {financial_years.data.map((row) => (
                                    <tr key={row.id}>
                                        <th>{row.id}</th>
                                        <th>{row.year}</th>
                                        <th>{row.value}</th>
                                        <th>{row.units}</th>
                                        <th>{row.variable_name}</th>
                                        <th>{row.variable_category}</th>
                                    </tr>
                                ))}
                            </tbody>
                        </table>
                    </div>
                </section>
            </main>
        </AuthenticatedLayout>
    );
}
