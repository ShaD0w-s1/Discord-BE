 SELECT CASE WHEN p.encrypted_password =? THEN 1 ELSE 0 END
 FROM tm_app_user u, tm_app_user_credential p
 WHERE u.app_user_uuid = p.app_user_uuid
 AND (LOWER(u.user_sign_in_nm) = ? OR LOWER(u.email) = ?)