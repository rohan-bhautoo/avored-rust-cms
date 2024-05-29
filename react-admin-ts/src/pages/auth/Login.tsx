import { useEffect, ChangeEvent } from "react";
import logo from "../../assets/logo_only.svg";
import { useNavigate, Link } from "react-router-dom";
import { isEmpty } from "lodash";
import InputField from "../../components/InputField";
import { useForm } from "react-hook-form";
import { joiResolver } from "@hookform/resolvers/joi";
import { useLogin } from "./hooks/useLogin";
import { loginSchema } from "./schemas/login.schema";
import _ from "lodash";
import { ErrorMessage } from "../../components/ErrorMessage";
import { useTranslation } from "react-i18next";

interface FormValues {
  [key: string]: any;
}

function Login() {
  const [t, i18] = useTranslation("global");
  const redirect = useNavigate();
  const { register, handleSubmit } = useForm<FormValues>({
    // Removed "formState: { errors }" from the object
    resolver: joiResolver(loginSchema),
  });
  const { mutate, isPending, error } = useLogin();

  const localeChange = (e: ChangeEvent<HTMLSelectElement>) => {
    i18.changeLanguage(e.target.value);
  };

  const isErrorExist = (key: string) => {
    return _.findIndex(
      _.get(error, "response.data.errors", []),
      (err: { key: string }) => err.key === key
    );
  };

  const getErrorMessage = (key: string) => {
    const errorIndex = isErrorExist(key);
    return _.get(error, `response.data.errors[${errorIndex}].message`);
  };

  // TODO: enhance to make this as an HOC
  // for "protecting" routes/pages
  useEffect(() => {
    /* to do make sure it execute once only..*/
    const token = localStorage.getItem("AUTH_TOKEN");
    if (!isEmpty(token)) {
      return redirect("/admin");
    }
  }, [redirect]); // Added "redirect" to the dependency array

  const submitHandler = (data: any) => {
    mutate(data);
  };

  return (
    <div className="min-h-screen bg-slate-100 flex flex-col justify-center py-12 sm:px-6 lg:px-8">
      <div className="flex justify-center">
        <img src={logo} className="w-20 h-20" alt="Avored Rust Cms" />
      </div>

      <div className="sm:mx-auto sm:w-full sm:max-w-md">
        <h2 className="mt-6 text-center text-3xl font-extrabold text-gray-900">
          {t("sign_into_your_account")}
        </h2>
      </div>
      <div></div>

      <div className="mt-8 sm:mx-auto sm:w-full sm:max-w-md">
        <div className="bg-white py-8 px-4 shadow sm:rounded-lg sm:px-10">
          <form onSubmit={handleSubmit(submitHandler)} className="space-y-5">
            <div>
              <InputField
                label={t("email_address")}
                type="text"
                name="email"
                autoFocus={true}
                register={register("email")}
                value={""}
                onChange={function (): void {
                  throw new Error("Function not implemented.");
                }}
                errorMessages={[]}
              />
              {isErrorExist("email") >= 0 && (
                <ErrorMessage message={getErrorMessage("email")} />
              )}
            </div>
            <div>
              <InputField
                label={t("password")}
                type="password"
                name="password"
                register={register("password")}
                value={""}
                onChange={function (): void {
                  throw new Error("Function not implemented.");
                }}
                errorMessages={[]}
              />
              {isErrorExist("password") >= 0 && (
                <ErrorMessage message={getErrorMessage("password")} />
              )}
            </div>
            <div className="flex items-center justify-end">
              <div className="text-sm">
                <Link
                  to={`/admin/forgot-password`}
                  className="font-medium text-primary-600 hover:text-primary-500"
                >
                  {t("forgot_your_password")}
                </Link>
              </div>
            </div>

            <div>
              <button
                type="submit"
                className="group relative bg-primary-600 w-full flex justify-center py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
              >
                {isPending ? "Loading..." : t("sign_in")}
              </button>
            </div>

            <div className="text-gray-600 text-center text-sm">
              {t("need_to_change_language")}
              <select
                onChange={(e) => localeChange(e)}
                className="outline-none border-none appearance-none pr-8"
              >
                <option>en</option>
                <option>fr</option>
              </select>
            </div>
          </form>
        </div>
      </div>
    </div>
  );
}

export default Login;
