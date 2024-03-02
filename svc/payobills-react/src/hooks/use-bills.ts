import { useEffect, useState } from "react"
import { getBills } from "../apis/get-bills";
import { BillProp } from "../components/Bill";

export const useBills = (deps?: React.DependencyList | undefined): [{
    error: string | null,
    bills: Array<BillProp>
}] => {
    const [bills, setBills] = useState<Array<BillProp>>([]);
    const [error, setError] = useState<string | null>(null);

    useEffect(() => {
        getBills({ limit: 10, offset: 0 }, (err: string | null, bills: Array<BillProp>) => {
            setError(err)
            setBills(bills)
        })
    }, deps)

    return [{ error, bills }]
}