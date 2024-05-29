import {
  useMutation,
  MutationFunction,
  MutationOptions,
  UseMutationResult,
} from "@tanstack/react-query";
import { AxiosInstance } from "axios";
import { useAxios } from "../../../hooks/useAxios";
import _ from "lodash";
import { useNavigate, NavigateFunction } from "react-router-dom";

interface RoleData {
  name: string;
  description: string;
  identifier: string;
  permissions: string[];
}

interface RoleResponse {
  status: boolean;
  data: {
    status: boolean;
  };
}

export const useUpdateRole = (
  role_id: string
): UseMutationResult<RoleResponse, Error, RoleData, unknown> => {
  const client: AxiosInstance = useAxios();
  const redirect: NavigateFunction = useNavigate();

  const mutationFn: MutationFunction<RoleResponse, RoleData> = async (
    data: RoleData
  ): Promise<RoleResponse> => {
    const url = "/role/" + role_id;
    const response = await client.put<RoleResponse>(url, JSON.stringify(data));
    return {
      status: response.data.status,
      data: { status: response.data.status },
    };
  };

  const mutationOptions: MutationOptions<
    RoleResponse,
    Error,
    RoleData,
    unknown
  > = {
    mutationFn,
    onSuccess: (res: RoleResponse) => {
      if (_.get(res, "data.status") === true) {
        redirect("/admin/role");
      }
    },
  };

  return useMutation(mutationOptions);
};
